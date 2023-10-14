use std::ffi::{c_char, c_void, CStr};

use anyhow::anyhow;

use crate::result::Result;

pub trait RmwCustomObjectPointer {
    const ID: &'static str;

    type Target;

    unsafe fn assert_implementation_identifier(&self) -> Result<()> {
        let ptr = self.implementation_identifier_as_ptr();
        if ptr.is_null() {
            Err(anyhow!(
                "{id}::implementation_identifier is null",
                id = <Self as RmwCustomObjectPointer>::ID,
            )
            .into())
        } else {
            match CStr::from_ptr(ptr).to_str() {
                Ok(id) => {
                    if id == crate::IMPLEMENTATION_IDENTIFIER {
                        Ok(())
                    } else {
                        Err(anyhow!(
                            "{id}::implementation_identifier is mismatched",
                            id = <Self as RmwCustomObjectPointer>::ID,
                        )
                        .into())
                    }
                }
                Err(error) => Err(anyhow!(
                    "{id}::implementation_identifier is invalid: {error}",
                    id = <Self as RmwCustomObjectPointer>::ID,
                )
                .into()),
            }
        }
    }

    fn data(&self) -> *const c_void;

    fn implementation_identifier_as_ptr(&self) -> *const c_char;

    unsafe fn cast(ptr: *const Self) -> Result<&'static <Self as RmwCustomObjectPointer>::Target> {
        <Self as RmwCustomObjectPointer>::try_cast(ptr).and_then(|ptr| {
            ptr.ok_or_else(|| {
                anyhow!(
                    "{id}::data is null",
                    id = <Self as RmwCustomObjectPointer>::ID
                )
                .into()
            })
        })
    }

    unsafe fn try_cast(
        ptr: *const Self,
    ) -> Result<Option<&'static <Self as RmwCustomObjectPointer>::Target>> {
        ptr.as_ref()
            .ok_or_else(|| {
                anyhow!("{id} is null", id = <Self as RmwCustomObjectPointer>::ID).into()
            })
            .and_then(|ptr| {
                ptr.assert_implementation_identifier().map(|()| {
                    ptr.data()
                        .cast::<<Self as RmwCustomObjectPointer>::Target>()
                        .as_ref()
                })
            })
    }
}

pub(crate) trait RmwCustomObjectPointerMut
where
    Self: RmwCustomObjectPointer,
{
    fn data_mut(&mut self) -> *mut c_void;

    unsafe fn cast_mut(
        ptr: *mut Self,
    ) -> Result<&'static mut <Self as RmwCustomObjectPointer>::Target> {
        <Self as RmwCustomObjectPointerMut>::try_cast_mut(ptr).and_then(|ptr| {
            ptr.ok_or_else(|| {
                anyhow!(
                    "{id}::data is null",
                    id = <Self as RmwCustomObjectPointer>::ID
                )
                .into()
            })
        })
    }

    unsafe fn try_cast_mut(
        ptr: *mut Self,
    ) -> Result<Option<&'static mut <Self as RmwCustomObjectPointer>::Target>> {
        ptr.as_mut()
            .ok_or_else(|| {
                anyhow!("{id} is null", id = <Self as RmwCustomObjectPointer>::ID).into()
            })
            .and_then(|ptr| {
                ptr.assert_implementation_identifier().map(|()| {
                    ptr.data_mut()
                        .cast::<<Self as RmwCustomObjectPointer>::Target>()
                        .as_mut()
                })
            })
    }
}

pub unsafe trait RmwGenericObjectPointer {
    const ID: &'static str;

    type Target;

    unsafe fn cast(ptr: *const Self) -> Result<&'static <Self as RmwGenericObjectPointer>::Target> {
        <Self as RmwGenericObjectPointer>::try_cast(ptr)?.ok_or_else(|| {
            anyhow!("{id} is null", id = <Self as RmwGenericObjectPointer>::ID).into()
        })
    }

    unsafe fn cast_mut(
        ptr: *mut Self,
    ) -> Result<&'static mut <Self as RmwGenericObjectPointer>::Target> {
        <Self as RmwGenericObjectPointer>::try_cast_mut(ptr)?.ok_or_else(|| {
            anyhow!("{id} is null", id = <Self as RmwGenericObjectPointer>::ID).into()
        })
    }

    unsafe fn try_cast(
        ptr: *const Self,
    ) -> Result<Option<&'static <Self as RmwGenericObjectPointer>::Target>>;

    unsafe fn try_cast_mut(
        ptr: *mut Self,
    ) -> Result<Option<&'static mut <Self as RmwGenericObjectPointer>::Target>>;
}

unsafe impl<T> RmwGenericObjectPointer for T
where
    Self: RmwCustomObjectPointerMut,
{
    const ID: &'static str = <Self as RmwCustomObjectPointer>::ID;

    type Target = <Self as RmwCustomObjectPointer>::Target;

    unsafe fn cast(ptr: *const Self) -> Result<&'static <Self as RmwGenericObjectPointer>::Target> {
        <Self as RmwCustomObjectPointer>::cast(ptr)
    }

    unsafe fn cast_mut(
        ptr: *mut Self,
    ) -> Result<&'static mut <Self as RmwGenericObjectPointer>::Target> {
        <Self as RmwCustomObjectPointerMut>::cast_mut(ptr)
    }

    unsafe fn try_cast(
        ptr: *const Self,
    ) -> Result<Option<&'static <Self as RmwGenericObjectPointer>::Target>> {
        <Self as RmwCustomObjectPointer>::try_cast(ptr)
    }

    unsafe fn try_cast_mut(
        ptr: *mut Self,
    ) -> Result<Option<&'static mut <Self as RmwGenericObjectPointer>::Target>> {
        <Self as RmwCustomObjectPointerMut>::try_cast_mut(ptr)
    }
}

pub unsafe fn cast<T>(ptr: *const T) -> Result<&'static <T as RmwGenericObjectPointer>::Target>
where
    T: RmwGenericObjectPointer,
{
    RmwGenericObjectPointer::cast(ptr)
}

pub unsafe fn cast_mut<T>(
    ptr: *mut T,
) -> Result<&'static mut <T as RmwGenericObjectPointer>::Target>
where
    T: RmwGenericObjectPointer,
{
    RmwGenericObjectPointer::cast_mut(ptr)
}

pub unsafe fn try_cast<T>(
    ptr: *mut T,
) -> Result<Option<&'static <T as RmwGenericObjectPointer>::Target>>
where
    T: RmwGenericObjectPointer,
{
    RmwGenericObjectPointer::try_cast(ptr)
}

pub unsafe fn try_cast_mut<T>(
    ptr: *mut T,
) -> Result<Option<&'static mut <T as RmwGenericObjectPointer>::Target>>
where
    T: RmwGenericObjectPointer,
{
    RmwGenericObjectPointer::try_cast_mut(ptr)
}

macro_rules! impl_generic_object_pointer {
    ( $( $ty:ty , )* ) => {
        $(
            unsafe impl RmwGenericObjectPointer for $ty {
                const ID: &'static str = stringify!($ty);

                type Target = Self;

                unsafe fn try_cast(
                    ptr: *const Self,
                ) -> Result<Option<&'static <Self as RmwGenericObjectPointer>::Target>> {
                    Ok(ptr.as_ref())
                }

                unsafe fn try_cast_mut(
                    ptr: *mut Self,
                ) -> Result<Option<&'static mut <Self as RmwGenericObjectPointer>::Target>> {
                    Ok(ptr.as_mut())
                }
            }
        )*
    };
}

impl_generic_object_pointer![
    bool,
    c_void,
    ::rmw_sys::rcutils_allocator_t,
    ::rmw_sys::rcutils_string_array_t,
    ::rmw_sys::rmw_clients_t,
    ::rmw_sys::rmw_context_t,
    ::rmw_sys::rmw_event_callback_t,
    ::rmw_sys::rmw_event_t,
    ::rmw_sys::rmw_events_t,
    ::rmw_sys::rmw_guard_conditions_t,
    ::rmw_sys::rmw_message_info_sequence_t,
    ::rmw_sys::rmw_message_info_t,
    ::rmw_sys::rmw_message_sequence_t,
    ::rmw_sys::rmw_publisher_allocation_t,
    ::rmw_sys::rmw_publisher_options_t,
    ::rmw_sys::rmw_qos_profile_t,
    ::rmw_sys::rmw_request_id_t,
    ::rmw_sys::rmw_serialized_message_t,
    ::rmw_sys::rmw_service_info_t,
    ::rmw_sys::rmw_services_t,
    ::rmw_sys::rmw_subscription_allocation_t,
    ::rmw_sys::rmw_subscription_content_filter_options_t,
    ::rmw_sys::rmw_subscription_options_t,
    ::rmw_sys::rmw_subscriptions_t,
    ::rmw_sys::rmw_time_t,
    ::rmw_sys::rmw_wait_set_t,
    ::rmw_sys::rosidl_message_type_support_t,
    ::rmw_sys::rosidl_runtime_c__Sequence__bound,
    ::rmw_sys::rosidl_service_type_support_t,
];
