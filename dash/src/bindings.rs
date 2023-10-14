use std::ffi::{c_char, c_void};

use rmw::{
    client::Client,
    ptr::{cast, cast_mut},
    result::{Result, RmwProcessResult, RmwValueResult},
};
use rmw_sys::{
    rcutils_allocator_t, rcutils_string_array_t, rmw_client_t, rmw_clients_t, rmw_context_t,
    rmw_event_callback_t, rmw_event_t, rmw_events_t, rmw_guard_conditions_t,
    rmw_message_info_sequence_t, rmw_message_info_t, rmw_message_sequence_t, rmw_node_t,
    rmw_publisher_allocation_t, rmw_publisher_options_t, rmw_publisher_t, rmw_qos_profile_t,
    rmw_request_id_t, rmw_ret_t, rmw_serialized_message_t, rmw_service_info_t, rmw_service_t,
    rmw_services_t, rmw_subscription_allocation_t, rmw_subscription_content_filter_options_t,
    rmw_subscription_options_t, rmw_subscription_t, rmw_subscriptions_t, rmw_time_t,
    rmw_wait_set_t, rosidl_message_type_support_t, rosidl_runtime_c__Sequence__bound,
    rosidl_service_type_support_t,
};

pub struct RmwExternImpl;

macro_rules! unsupported {
    ( $function:ident ) => {{
        ::tracing::warn!(concat!(
            stringify!($function),
            " is not implemented for rmw_dash"
        ));
        ::rmw_sys::RMW_RET_UNSUPPORTED
    }};
}

macro_rules! planned {
    ( $function:ident ) => {{
        ::tracing::warn!(concat!(
            stringify!($function),
            " is not implemented for rmw_dash yet"
        ));
        ::rmw_sys::RMW_RET_UNSUPPORTED
    }};
}

unsafe impl ::rmw_sys::RmwExtern for RmwExternImpl {
    #[no_mangle]
    unsafe extern "C" fn rmw_borrow_loaned_message(
        publisher: *const rmw_publisher_t,
        type_support: *const rosidl_message_type_support_t,
        ros_message: *mut *mut c_void,
    ) -> rmw_ret_t {
        unsupported!(rmw_borrow_loaned_message)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_client_request_publisher_get_actual_qos(
        client: *const rmw_client_t,
        qos: *mut rmw_qos_profile_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_client_request_publisher_get_actual_qos)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_client_response_subscription_get_actual_qos(
        client: *const rmw_client_t,
        qos: *mut rmw_qos_profile_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_client_response_subscription_get_actual_qos)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_client_set_on_new_response_callback(
        client: *mut rmw_client_t,
        callback: rmw_event_callback_t,
        user_data: *const c_void,
    ) -> rmw_ret_t {
        unsupported!(rmw_client_set_on_new_response_callback)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_count_clients(
        node: *const rmw_node_t,
        service_name: *const c_char,
        count: *mut usize,
    ) -> rmw_ret_t {
        unsupported!(rmw_count_clients)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_count_publishers(
        node: *const rmw_node_t,
        topic_name: *const c_char,
        count: *mut usize,
    ) -> rmw_ret_t {
        planned!(rmw_count_publishers)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_count_services(
        node: *const rmw_node_t,
        service_name: *const c_char,
        count: *mut usize,
    ) -> rmw_ret_t {
        unsupported!(rmw_count_services)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_count_subscribers(
        node: *const rmw_node_t,
        topic_name: *const c_char,
        count: *mut usize,
    ) -> rmw_ret_t {
        planned!(rmw_count_subscribers)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_create_client(
        node: *const rmw_node_t,
        type_support: *const rosidl_service_type_support_t,
        service_name: *const c_char,
        qos_policies: *const rmw_qos_profile_t,
    ) -> *mut rmw_client_t {
        Client::new_ptr(node, qos_policies, service_name, type_support)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_create_node(
        context: *mut rmw_context_t,
        name: *const c_char,
        namespace_: *const c_char,
    ) -> *mut rmw_node_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_create_publisher(
        node: *const rmw_node_t,
        type_support: *const rosidl_message_type_support_t,
        topic_name: *const c_char,
        qos_profile: *const rmw_qos_profile_t,
        publisher_options: *const rmw_publisher_options_t,
    ) -> *mut rmw_publisher_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_create_service(
        node: *const rmw_node_t,
        type_support: *const rosidl_service_type_support_t,
        service_name: *const c_char,
        qos_profile: *const rmw_qos_profile_t,
    ) -> *mut rmw_service_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_create_subscription(
        node: *const rmw_node_t,
        type_support: *const rosidl_message_type_support_t,
        topic_name: *const c_char,
        qos_policies: *const rmw_qos_profile_t,
        subscription_options: *const rmw_subscription_options_t,
    ) -> *mut rmw_subscription_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_deserialize(
        serialized_message: *const rmw_serialized_message_t,
        type_support: *const rosidl_message_type_support_t,
        ros_message: *mut c_void,
    ) -> rmw_ret_t {
        planned!(rmw_deserialize)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_event_set_callback(
        event: *mut rmw_event_t,
        callback: rmw_event_callback_t,
        user_data: *const c_void,
    ) -> rmw_ret_t {
        unsupported!(rmw_event_set_callback)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_fini_publisher_allocation(
        allocation: *mut rmw_publisher_allocation_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_fini_publisher_allocation)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_fini_subscription_allocation(
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_fini_subscription_allocation)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_get_node_names(
        node: *const rmw_node_t,
        node_names: *mut rcutils_string_array_t,
        node_namespaces: *mut rcutils_string_array_t,
    ) -> rmw_ret_t {
        planned!(rmw_get_node_names)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_get_node_names_with_enclaves(
        node: *const rmw_node_t,
        node_names: *mut rcutils_string_array_t,
        node_namespaces: *mut rcutils_string_array_t,
        enclaves: *mut rcutils_string_array_t,
    ) -> rmw_ret_t {
        planned!(rmw_get_node_names_with_enclaves)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_get_serialized_message_size(
        type_support: *const rosidl_message_type_support_t,
        message_bounds: *const rosidl_runtime_c__Sequence__bound,
        size: *mut usize,
    ) -> rmw_ret_t {
        planned!(rmw_get_serialized_message_size)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_init_publisher_allocation(
        type_support: *const rosidl_message_type_support_t,
        message_bounds: *const rosidl_runtime_c__Sequence__bound,
        allocation: *mut rmw_publisher_allocation_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_init_publisher_allocation)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_init_subscription_allocation(
        type_support: *const rosidl_message_type_support_t,
        message_bounds: *const rosidl_runtime_c__Sequence__bound,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_init_subscription_allocation)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_publish(
        publisher: *const rmw_publisher_t,
        ros_message: *const c_void,
        allocation: *mut rmw_publisher_allocation_t,
    ) -> rmw_ret_t {
        planned!(rmw_publisher_t)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_publish_loaned_message(
        publisher: *const rmw_publisher_t,
        ros_message: *mut c_void,
        allocation: *mut rmw_publisher_allocation_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_publish_loaned_message)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_publish_serialized_message(
        publisher: *const rmw_publisher_t,
        serialized_message: *const rmw_serialized_message_t,
        allocation: *mut rmw_publisher_allocation_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_publish_serialized_message)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_publisher_count_matched_subscriptions(
        publisher: *const rmw_publisher_t,
        subscription_count: *mut usize,
    ) -> rmw_ret_t {
        planned!(rmw_publisher_count_matched_subscriptions)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_publisher_get_actual_qos(
        publisher: *const rmw_publisher_t,
        qos: *mut rmw_qos_profile_t,
    ) -> rmw_ret_t {
        planned!(rmw_publisher_get_actual_qos)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_publisher_wait_for_all_acked(
        publisher: *const rmw_publisher_t,
        wait_timeout: rmw_time_t,
    ) -> rmw_ret_t {
        planned!(rmw_publisher_wait_for_all_acked)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_return_loaned_message_from_publisher(
        publisher: *const rmw_publisher_t,
        loaned_message: *mut c_void,
    ) -> rmw_ret_t {
        unsupported!(rmw_return_loaned_message_from_publisher)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_return_loaned_message_from_subscription(
        subscription: *const rmw_subscription_t,
        loaned_message: *mut c_void,
    ) -> rmw_ret_t {
        unsupported!(rmw_return_loaned_message_from_subscription)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_send_request(
        client: *const rmw_client_t,
        ros_request: *const c_void,
        sequence_id: *mut i64,
    ) -> rmw_ret_t {
        planned!(rmw_send_request)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_send_response(
        service: *const rmw_service_t,
        request_header: *mut rmw_request_id_t,
        ros_response: *mut c_void,
    ) -> rmw_ret_t {
        planned!(rmw_send_response)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_serialize(
        ros_message: *const c_void,
        type_support: *const rosidl_message_type_support_t,
        serialized_message: *mut rmw_serialized_message_t,
    ) -> rmw_ret_t {
        planned!(rmw_serialize)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_service_request_subscription_get_actual_qos(
        service: *const rmw_service_t,
        qos: *mut rmw_qos_profile_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_service_request_subscription_get_actual_qos)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_service_response_publisher_get_actual_qos(
        service: *const rmw_service_t,
        qos: *mut rmw_qos_profile_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_service_response_publisher_get_actual_qos)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_service_server_is_available(
        node: *const rmw_node_t,
        client: *const rmw_client_t,
        is_available: *mut bool,
    ) -> rmw_ret_t {
        planned!(rmw_service_server_is_available)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_service_set_on_new_request_callback(
        service: *mut rmw_service_t,
        callback: rmw_event_callback_t,
        user_data: *const c_void,
    ) -> rmw_ret_t {
        unsupported!(rmw_service_set_on_new_request_callback)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_subscription_count_matched_publishers(
        subscription: *const rmw_subscription_t,
        publisher_count: *mut usize,
    ) -> rmw_ret_t {
        planned!(rmw_subscription_count_matched_publishers)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_subscription_get_actual_qos(
        subscription: *const rmw_subscription_t,
        qos: *mut rmw_qos_profile_t,
    ) -> rmw_ret_t {
        planned!(rmw_subscription_get_actual_qos)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_subscription_get_content_filter(
        subscription: *const rmw_subscription_t,
        allocator: *mut rcutils_allocator_t,
        options: *mut rmw_subscription_content_filter_options_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_subscription_get_content_filter)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_subscription_set_content_filter(
        subscription: *mut rmw_subscription_t,
        options: *const rmw_subscription_content_filter_options_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_subscription_set_content_filter)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_subscription_set_on_new_message_callback(
        subscription: *mut rmw_subscription_t,
        callback: rmw_event_callback_t,
        user_data: *const c_void,
    ) -> rmw_ret_t {
        unsupported!(rmw_subscription_set_on_new_message_callback)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take(
        subscription: *const rmw_subscription_t,
        ros_message: *mut c_void,
        taken: *mut bool,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        planned!(rmw_take)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_loaned_message(
        subscription: *const rmw_subscription_t,
        loaned_message: *mut *mut c_void,
        taken: *mut bool,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_take_loaned_message)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_loaned_message_with_info(
        subscription: *const rmw_subscription_t,
        loaned_message: *mut *mut c_void,
        taken: *mut bool,
        message_info: *mut rmw_message_info_t,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        unsupported!(rmw_take_loaned_message_with_info)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_request(
        service: *const rmw_service_t,
        request_header: *mut rmw_service_info_t,
        ros_request: *mut c_void,
        taken: *mut bool,
    ) -> rmw_ret_t {
        planned!(rmw_take_request)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_response(
        client: *const rmw_client_t,
        request_header: *mut rmw_service_info_t,
        ros_response: *mut c_void,
        taken: *mut bool,
    ) -> rmw_ret_t {
        unsafe fn prepare_type_and_execute(
            client: *const rmw_client_t,
            request_header: *mut rmw_service_info_t,
            ros_response: *mut c_void,
            taken: *mut bool,
        ) -> Result<()> {
            let client = cast(client)?;
            let request_header = cast_mut(request_header)?;
            let ros_response = cast_mut(ros_response)?;
            let taken = cast_mut(taken)?;
            execute(client, request_header, ros_response, taken)
        }

        unsafe fn execute(
            client: &Client,
            request_header: &mut rmw_service_info_t,
            ros_response: &mut c_void,
            taken: &mut bool,
        ) -> Result<()> {
            ::tracing::warn!(concat!(
                "rmw_take_response",
                " is not implemented for rmw_dash"
            ));
            Err(::rmw::error::Error::Unsupported)
        }

        prepare_type_and_execute(client, request_header, ros_response, taken).finish()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_sequence(
        subscription: *const rmw_subscription_t,
        count: usize,
        message_sequence: *mut rmw_message_sequence_t,
        message_info_sequence: *mut rmw_message_info_sequence_t,
        taken: *mut usize,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        planned!(rmw_take_sequence)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_serialized_message(
        subscription: *const rmw_subscription_t,
        serialized_message: *mut rmw_serialized_message_t,
        taken: *mut bool,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        planned!(rmw_take_serialized_message)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_serialized_message_with_info(
        subscription: *const rmw_subscription_t,
        serialized_message: *mut rmw_serialized_message_t,
        taken: *mut bool,
        message_info: *mut rmw_message_info_t,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        planned!(rmw_take_serialized_message_with_info)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_with_info(
        subscription: *const rmw_subscription_t,
        ros_message: *mut c_void,
        taken: *mut bool,
        message_info: *mut rmw_message_info_t,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        planned!(rmw_take_with_info)
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_wait(
        subscriptions: *mut rmw_subscriptions_t,
        guard_conditions: *mut rmw_guard_conditions_t,
        services: *mut rmw_services_t,
        clients: *mut rmw_clients_t,
        events: *mut rmw_events_t,
        wait_set: *mut rmw_wait_set_t,
        wait_timeout: *const rmw_time_t,
    ) -> rmw_ret_t {
        planned!(rmw_wait)
    }
}
