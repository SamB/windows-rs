#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAudioFrameNative(::windows_core::IUnknown);
impl IAudioFrameNative {
    pub unsafe fn GetData<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetData)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IAudioFrameNative, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IAudioFrameNative {
    type Vtable = IAudioFrameNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAudioFrameNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20be1e2e_930f_4746_9335_3c332f255093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNative_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAudioFrameNativeFactory(::windows_core::IUnknown);
impl IAudioFrameNativeFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateFromMFSample<P0, P1, T>(&self, data: P0, forcereadonly: P1) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::Media::MediaFoundation::IMFSample>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateFromMFSample)(::windows_core::Interface::as_raw(self), data.into_param().abi(), forcereadonly.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IAudioFrameNativeFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IAudioFrameNativeFactory {
    type Vtable = IAudioFrameNativeFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAudioFrameNativeFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7bd67cf8_bf7d_43e6_af8d_b170ee0c0110);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNativeFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub CreateFromMFSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, forcereadonly: super::super::super::Foundation::BOOL, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    CreateFromMFSample: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVideoFrameNative(::windows_core::IUnknown);
impl IVideoFrameNative {
    pub unsafe fn GetData<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetData)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetDevice)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IVideoFrameNative, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IVideoFrameNative {
    type Vtable = IVideoFrameNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVideoFrameNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26ba702b_314a_4620_aaf6_7a51aa58fa18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNative_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVideoFrameNativeFactory(::windows_core::IUnknown);
impl IVideoFrameNativeFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateFromMFSample<P0, P1, P2, T>(&self, data: P0, subtype: *const ::windows_core::GUID, width: u32, height: u32, forcereadonly: P1, mindisplayaperture: ::core::option::Option<*const super::super::super::Media::MediaFoundation::MFVideoArea>, device: P2) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::Media::MediaFoundation::IMFSample>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::super::Media::MediaFoundation::IMFDXGIDeviceManager>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateFromMFSample)(::windows_core::Interface::as_raw(self), data.into_param().abi(), subtype, width, height, forcereadonly.into_param().abi(), ::core::mem::transmute(mindisplayaperture.unwrap_or(::std::ptr::null())), device.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IVideoFrameNativeFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IVideoFrameNativeFactory {
    type Vtable = IVideoFrameNativeFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVideoFrameNativeFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69e3693e_8e1e_4e63_ac4c_7fdc21d9731d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNativeFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub CreateFromMFSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, subtype: *const ::windows_core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    CreateFromMFSample: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
pub const CLSID_AudioFrameNativeFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16a0a3b9_9f65_4102_9367_2cda3a4f372a);
#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
pub const CLSID_VideoFrameNativeFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd194386a_04e3_4814_8100_b2b0ae6d78c7);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
