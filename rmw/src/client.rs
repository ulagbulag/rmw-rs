use std::ffi::{c_char, c_void, CStr};

use anyhow::Result;
use rmw_sys::{
    rmw_client_s, rmw_client_t, rmw_node_t, rmw_qos_profile_t, rosidl_service_type_support_t,
};

use crate::ptr::{RmwCustomObjectPointer, RmwCustomObjectPointerMut};

pub struct Client {
    node: *const rmw_node_t,
    qos_policies: *const rmw_qos_profile_t,
    service_name: *const c_char,
    type_support: *const rosidl_service_type_support_t,
}

impl Client {
    fn new(
        node: *const rmw_node_t,
        qos_policies: *const rmw_qos_profile_t,
        service_name: *const c_char,
        type_support: *const rosidl_service_type_support_t,
    ) -> Self {
        Self {
            node,
            qos_policies,
            service_name,
            type_support,
        }
    }

    pub unsafe fn new_ptr(
        node: *const rmw_node_t,
        qos_policies: *const rmw_qos_profile_t,
        service_name: *const c_char,
        type_support: *const rosidl_service_type_support_t,
    ) -> *mut rmw_client_t {
        Box::new(Self::new(node, qos_policies, service_name, type_support)).into_ptr()
    }

    unsafe fn into_ptr(self: Box<Self>) -> *mut rmw_client_t {
        Box::leak(Box::new(rmw_client_s {
            implementation_identifier: CStr::from_bytes_with_nul_unchecked(
                crate::IMPLEMENTATION_IDENTIFIER.as_bytes(),
            )
            .as_ptr(),
            service_name: self.service_name,
            data: Box::leak(self) as *mut Self as *mut c_void,
        }))
    }
}

impl RmwCustomObjectPointer for rmw_client_t {
    const ID: &'static str = "rmw_client_t";

    type Target = Client;

    fn data(&self) -> *const c_void {
        self.data
    }

    fn implementation_identifier_as_ptr(&self) -> *const c_char {
        self.implementation_identifier
    }
}

impl RmwCustomObjectPointerMut for rmw_client_t {
    fn data_mut(&mut self) -> *mut c_void {
        self.data
    }
}
