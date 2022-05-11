//! Cantrip OS process management support

#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::string::String;
use alloc::boxed::Box;
use cantrip_memory_interface::ObjDescBundle;
use cantrip_os_common::slot_allocator;
use cantrip_proc_interface::Bundle;
use cantrip_proc_interface::BundleImplInterface;
use cantrip_proc_interface::BundleIdArray;
use cantrip_proc_interface::PackageManagementInterface;
use cantrip_proc_interface::ProcessControlInterface;
use cantrip_proc_interface::ProcessManagerError;
use cantrip_proc_interface::ProcessManagerInterface;
use cantrip_security_interface::cantrip_security_install;
use cantrip_security_interface::cantrip_security_load_application;
use cantrip_security_interface::cantrip_security_uninstall;
use log::trace;
use spin::Mutex;

use slot_allocator::CSpaceSlot;

mod sel4bundle;
use sel4bundle::seL4BundleImpl;

mod proc_manager;
pub use proc_manager::ProcessManager;

// NB: CANTRIP_PROC cannot be used before setup is completed with a call to init()
#[cfg(not(test))]
pub static mut CANTRIP_PROC: CantripProcManager = CantripProcManager::empty();

// CantripProcManager bundles an instance of the ProcessManager that operates
// on CantripOS interfaces and synchronizes public use with a Mutex. There is
// a two-step dance to setup an instance because we want CANTRIP_PROC static
// and ProcessManager is incapable of supplying a const fn due it's use of
// hashbrown::HashMap.
pub struct CantripProcManager {
    manager: Mutex<Option<ProcessManager>>,
}
impl CantripProcManager {
    // Constructs a partially-initialized instance; to complete call init().
    // This is needed because we need a const fn for static setup and with
    // that constraint we cannot reference self.interface.
    const fn empty() -> CantripProcManager {
        CantripProcManager {
            manager: Mutex::new(None),
        }
    }

    // Finishes the setup started by empty():
    pub fn init(&self) {
        *self.manager.lock() = Some(ProcessManager::new(CantripManagerInterface));
    }

    // Returns the bundle capacity.
    pub fn capacity(&self) -> usize {
        self.manager.lock().as_ref().unwrap().capacity()
    }
}
// These just lock accesses and handle the necessary indirection.
impl PackageManagementInterface for CantripProcManager {
    fn install(
        &mut self,
        pkg_contents: &ObjDescBundle,
    ) -> Result<String, ProcessManagerError> {
        self.manager
            .lock()
            .as_mut()
            .unwrap()
            .install(pkg_contents)
    }
    fn uninstall(&mut self, bundle_id: &str) -> Result<(), ProcessManagerError> {
        self.manager.lock().as_mut().unwrap().uninstall(bundle_id)
    }
}
impl ProcessControlInterface for CantripProcManager {
    fn start(&mut self, bundle_id: &str) -> Result<(), ProcessManagerError> {
        self.manager.lock().as_mut().unwrap().start(bundle_id)
    }
    fn stop(&mut self, bundle_id: &str) -> Result<(), ProcessManagerError> {
        self.manager.lock().as_mut().unwrap().stop(bundle_id)
    }
    fn get_running_bundles(&self) -> Result<BundleIdArray, ProcessManagerError> {
        self.manager.lock().as_ref().unwrap().get_running_bundles()
    }
}

struct CantripManagerInterface;
impl ProcessManagerInterface for CantripManagerInterface {
    fn install(
        &mut self,
        pkg_contents: &ObjDescBundle,
    ) -> Result<String, ProcessManagerError> {
        trace!("ProcessManagerInterface::install pkg_contents {}", pkg_contents);

        // Package contains: application manifest, application binary, and
        // (optional) ML workload binary to run on vector core.
        // Manifest contains bundle_id.
        // Resulting flash file/pathname is fixed (1 app / bundle), only need bundle_id.
        // Pass opaque package contents through; get back bundle_id.

        // This is handled by the SecurityCoordinator.
        Ok(cantrip_security_install(pkg_contents)?)
    }
    fn uninstall(&mut self, bundle_id: &str) -> Result<(), ProcessManagerError> {
        trace!("ProcessManagerInterface::uninstall bundle_id {}", bundle_id);

        // NB: the caller has already checked no running application exists
        // NB: the Security Core is assumed to invalidate/remove any kv store

        // This is handled by the SecurityCoordinator.
        Ok(cantrip_security_uninstall(bundle_id)?)
    }
    fn start(&mut self, bundle: &Bundle) -> Result<Box<dyn BundleImplInterface>, ProcessManagerError> {
        trace!("ProcessManagerInterface::start {:?}", bundle);

        // Design doc says:
        // 1. Ask security core for application footprint with SizeBuffer
        // 2. Ask security core for manifest (maybe piggyback on SizeBuffer)
        //    and parse for necessary info (e.g. whether kv Storage is
        //    required, other privileges/capabilities)
        // 3. Ask MemoryManager for shared memory pages for the application
        //    (model handled separately by MlCoordinator since we do not know
        //    which model will be used)
        // 4. Allocate other seL4 resources:
        //     - VSpace, TCB & necessary capabiltiies
        // 5. Ask security core to VerifyAndLoad app into shared memory pages
        // 6. Complete seL4 setup:
        //     - Setup application system context and start thread
        //     - Badge seL4 recv cap w/ bundle_id for (optional) StorageManager
        //       access
        // What we do atm is:
        // 1. Ask SecurityCoordinator to return the application contents to load.
        //    Data are delivered as a read-only ObjDescBundle ready to copy into
        //    the VSpace.
        // 2. Do 4+6 with BundleImplInterface::start.

        // TODO(sleffler): awkward container_slot ownership
        let mut container_slot = CSpaceSlot::new();
        let bundle_frames =
            cantrip_security_load_application(&bundle.app_id, &container_slot)?;
        let mut sel4_bundle = seL4BundleImpl::new(bundle, &bundle_frames)?;
        sel4_bundle.start()?;

        // sel4_bundle owns container_slot now; release our ref so it's not
        // reclaimed when container_slot goes out of scope.
        container_slot.release();

        Ok(Box::new(sel4_bundle) as _)
    }
    fn stop(&mut self, bundle_impl: &mut dyn BundleImplInterface) -> Result<(), ProcessManagerError> {
        trace!("ProcessManagerInterface::stop");

        // 0. Assume thread is running (caller verifies)
        // 1. Notify application so it can do cleanup; e.g. ask the
        //    MLCoordinator to stop any ML workloads
        // 2. Wait some period of time for an ack from application
        // 3. Stop thread
        // 4. Reclaim seL4 resources: TCB, VSpace, memory, capabilities, etc.
        // TODO(sleffler): fill-in 1+2
        bundle_impl.stop()
    }
}
