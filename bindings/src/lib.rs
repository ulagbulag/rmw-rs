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

pub struct Rwm {}

unsafe impl ::rmw_sys::RmwExtern for Rwm {
    #[no_mangle]
    unsafe extern "C" fn rmw_borrow_loaned_message(
        publisher: *const rmw_publisher_t,
        type_support: *const rosidl_message_type_support_t,
        ros_message: *mut *mut ::std::os::raw::c_void,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_client_request_publisher_get_actual_qos(
        client: *const rmw_client_t,
        qos: *mut rmw_qos_profile_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_client_response_subscription_get_actual_qos(
        client: *const rmw_client_t,
        qos: *mut rmw_qos_profile_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_client_set_on_new_response_callback(
        client: *mut rmw_client_t,
        callback: rmw_event_callback_t,
        user_data: *const ::std::os::raw::c_void,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_count_clients(
        node: *const rmw_node_t,
        service_name: *const ::std::os::raw::c_char,
        count: *mut usize,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_count_publishers(
        node: *const rmw_node_t,
        topic_name: *const ::std::os::raw::c_char,
        count: *mut usize,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_count_services(
        node: *const rmw_node_t,
        service_name: *const ::std::os::raw::c_char,
        count: *mut usize,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_count_subscribers(
        node: *const rmw_node_t,
        topic_name: *const ::std::os::raw::c_char,
        count: *mut usize,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_create_client(
        node: *const rmw_node_t,
        type_support: *const rosidl_service_type_support_t,
        service_name: *const ::std::os::raw::c_char,
        qos_policies: *const rmw_qos_profile_t,
    ) -> *mut rmw_client_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_create_node(
        context: *mut rmw_context_t,
        name: *const ::std::os::raw::c_char,
        namespace_: *const ::std::os::raw::c_char,
    ) -> *mut rmw_node_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_create_publisher(
        node: *const rmw_node_t,
        type_support: *const rosidl_message_type_support_t,
        topic_name: *const ::std::os::raw::c_char,
        qos_profile: *const rmw_qos_profile_t,
        publisher_options: *const rmw_publisher_options_t,
    ) -> *mut rmw_publisher_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_create_service(
        node: *const rmw_node_t,
        type_support: *const rosidl_service_type_support_t,
        service_name: *const ::std::os::raw::c_char,
        qos_profile: *const rmw_qos_profile_t,
    ) -> *mut rmw_service_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_create_subscription(
        node: *const rmw_node_t,
        type_support: *const rosidl_message_type_support_t,
        topic_name: *const ::std::os::raw::c_char,
        qos_policies: *const rmw_qos_profile_t,
        subscription_options: *const rmw_subscription_options_t,
    ) -> *mut rmw_subscription_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_deserialize(
        serialized_message: *const rmw_serialized_message_t,
        type_support: *const rosidl_message_type_support_t,
        ros_message: *mut ::std::os::raw::c_void,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_event_set_callback(
        event: *mut rmw_event_t,
        callback: rmw_event_callback_t,
        user_data: *const ::std::os::raw::c_void,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_fini_publisher_allocation(
        allocation: *mut rmw_publisher_allocation_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_fini_subscription_allocation(
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_get_node_names(
        node: *const rmw_node_t,
        node_names: *mut rcutils_string_array_t,
        node_namespaces: *mut rcutils_string_array_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_get_node_names_with_enclaves(
        node: *const rmw_node_t,
        node_names: *mut rcutils_string_array_t,
        node_namespaces: *mut rcutils_string_array_t,
        enclaves: *mut rcutils_string_array_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_get_serialized_message_size(
        type_support: *const rosidl_message_type_support_t,
        message_bounds: *const rosidl_runtime_c__Sequence__bound,
        size: *mut usize,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_init_publisher_allocation(
        type_support: *const rosidl_message_type_support_t,
        message_bounds: *const rosidl_runtime_c__Sequence__bound,
        allocation: *mut rmw_publisher_allocation_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_init_subscription_allocation(
        type_support: *const rosidl_message_type_support_t,
        message_bounds: *const rosidl_runtime_c__Sequence__bound,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_publish(
        publisher: *const rmw_publisher_t,
        ros_message: *const ::std::os::raw::c_void,
        allocation: *mut rmw_publisher_allocation_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_publish_loaned_message(
        publisher: *const rmw_publisher_t,
        ros_message: *mut ::std::os::raw::c_void,
        allocation: *mut rmw_publisher_allocation_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_publish_serialized_message(
        publisher: *const rmw_publisher_t,
        serialized_message: *const rmw_serialized_message_t,
        allocation: *mut rmw_publisher_allocation_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_publisher_count_matched_subscriptions(
        publisher: *const rmw_publisher_t,
        subscription_count: *mut usize,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_publisher_get_actual_qos(
        publisher: *const rmw_publisher_t,
        qos: *mut rmw_qos_profile_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_publisher_wait_for_all_acked(
        publisher: *const rmw_publisher_t,
        wait_timeout: rmw_time_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_return_loaned_message_from_publisher(
        publisher: *const rmw_publisher_t,
        loaned_message: *mut ::std::os::raw::c_void,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_return_loaned_message_from_subscription(
        subscription: *const rmw_subscription_t,
        loaned_message: *mut ::std::os::raw::c_void,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_send_request(
        client: *const rmw_client_t,
        ros_request: *const ::std::os::raw::c_void,
        sequence_id: *mut i64,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_send_response(
        service: *const rmw_service_t,
        request_header: *mut rmw_request_id_t,
        ros_response: *mut ::std::os::raw::c_void,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_serialize(
        ros_message: *const ::std::os::raw::c_void,
        type_support: *const rosidl_message_type_support_t,
        serialized_message: *mut rmw_serialized_message_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_service_request_subscription_get_actual_qos(
        service: *const rmw_service_t,
        qos: *mut rmw_qos_profile_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_service_response_publisher_get_actual_qos(
        service: *const rmw_service_t,
        qos: *mut rmw_qos_profile_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_service_server_is_available(
        node: *const rmw_node_t,
        client: *const rmw_client_t,
        is_available: *mut bool,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_service_set_on_new_request_callback(
        service: *mut rmw_service_t,
        callback: rmw_event_callback_t,
        user_data: *const ::std::os::raw::c_void,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_subscription_count_matched_publishers(
        subscription: *const rmw_subscription_t,
        publisher_count: *mut usize,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_subscription_get_actual_qos(
        subscription: *const rmw_subscription_t,
        qos: *mut rmw_qos_profile_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_subscription_get_content_filter(
        subscription: *const rmw_subscription_t,
        allocator: *mut rcutils_allocator_t,
        options: *mut rmw_subscription_content_filter_options_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_subscription_set_content_filter(
        subscription: *mut rmw_subscription_t,
        options: *const rmw_subscription_content_filter_options_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_subscription_set_on_new_message_callback(
        subscription: *mut rmw_subscription_t,
        callback: rmw_event_callback_t,
        user_data: *const ::std::os::raw::c_void,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take(
        subscription: *const rmw_subscription_t,
        ros_message: *mut ::std::os::raw::c_void,
        taken: *mut bool,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_loaned_message(
        subscription: *const rmw_subscription_t,
        loaned_message: *mut *mut ::std::os::raw::c_void,
        taken: *mut bool,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_loaned_message_with_info(
        subscription: *const rmw_subscription_t,
        loaned_message: *mut *mut ::std::os::raw::c_void,
        taken: *mut bool,
        message_info: *mut rmw_message_info_t,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_request(
        service: *const rmw_service_t,
        request_header: *mut rmw_service_info_t,
        ros_request: *mut ::std::os::raw::c_void,
        taken: *mut bool,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_response(
        client: *const rmw_client_t,
        request_header: *mut rmw_service_info_t,
        ros_response: *mut ::std::os::raw::c_void,
        taken: *mut bool,
    ) -> rmw_ret_t {
        todo!()
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
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_serialized_message(
        subscription: *const rmw_subscription_t,
        serialized_message: *mut rmw_serialized_message_t,
        taken: *mut bool,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_serialized_message_with_info(
        subscription: *const rmw_subscription_t,
        serialized_message: *mut rmw_serialized_message_t,
        taken: *mut bool,
        message_info: *mut rmw_message_info_t,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        todo!()
    }

    #[no_mangle]
    unsafe extern "C" fn rmw_take_with_info(
        subscription: *const rmw_subscription_t,
        ros_message: *mut ::std::os::raw::c_void,
        taken: *mut bool,
        message_info: *mut rmw_message_info_t,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> rmw_ret_t {
        todo!()
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
        todo!()
    }
}
