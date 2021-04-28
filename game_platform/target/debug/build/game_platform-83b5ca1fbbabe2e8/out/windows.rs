#[allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Windows {
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Win32 {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod SystemServices {
            #[repr(transparent)]
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct PSTR(pub *mut u8);
            impl PSTR {
                pub const NULL: Self = Self(::std::ptr::null_mut());
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::std::default::Default for PSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            impl ::std::cmp::PartialEq for PSTR {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            unsafe impl ::windows::Abi for PSTR {
                type Abi = Self;
                fn drop_param(param: &mut ::windows::Param<Self>) {
                    if let ::windows::Param::Boxed(value) = param {
                        if !value.0.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for &'a str {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for String {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod WindowsAndMessaging {
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: marker :: Copy,
                :: std :: clone :: Clone,
                :: std :: default :: Default,
                :: std :: fmt :: Debug,
            )]
            #[repr(transparent)]
            pub struct MESSAGEBOX_RESULT(pub i32);
            impl MESSAGEBOX_RESULT {
                pub const IDOK: Self = Self(1i32);
                pub const IDCANCEL: Self = Self(2i32);
                pub const IDABORT: Self = Self(3i32);
                pub const IDRETRY: Self = Self(4i32);
                pub const IDIGNORE: Self = Self(5i32);
                pub const IDYES: Self = Self(6i32);
                pub const IDNO: Self = Self(7i32);
                pub const IDCLOSE: Self = Self(8i32);
                pub const IDHELP: Self = Self(9i32);
                pub const IDTRYAGAIN: Self = Self(10i32);
                pub const IDCONTINUE: Self = Self(11i32);
                pub const IDASYNC: Self = Self(32001i32);
                pub const IDTIMEOUT: Self = Self(32000i32);
            }
            impl ::std::convert::From<i32> for MESSAGEBOX_RESULT {
                fn from(value: i32) -> Self {
                    Self(value)
                }
            }
            unsafe impl ::windows::Abi for MESSAGEBOX_RESULT {
                type Abi = Self;
            }
            #[repr(transparent)]
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            pub struct HWND(pub isize);
            impl HWND {}
            impl ::std::default::Default for HWND {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl HWND {
                pub const NULL: Self = Self(0);
                pub fn is_null(&self) -> bool {
                    self.0 == 0
                }
            }
            impl ::std::fmt::Debug for HWND {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("HWND")
                        .field("Value", &format_args!("{:?}", self.0))
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for HWND {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for HWND {}
            unsafe impl ::windows::Abi for HWND {
                type Abi = Self;
            }
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: marker :: Copy,
                :: std :: clone :: Clone,
                :: std :: default :: Default,
                :: std :: fmt :: Debug,
            )]
            #[repr(transparent)]
            pub struct MESSAGEBOX_STYLE(pub u32);
            impl MESSAGEBOX_STYLE {
                pub const MB_ABORTRETRYIGNORE: Self = Self(2u32);
                pub const MB_CANCELTRYCONTINUE: Self = Self(6u32);
                pub const MB_HELP: Self = Self(16384u32);
                pub const MB_OK: Self = Self(0u32);
                pub const MB_OKCANCEL: Self = Self(1u32);
                pub const MB_RETRYCANCEL: Self = Self(5u32);
                pub const MB_YESNO: Self = Self(4u32);
                pub const MB_YESNOCANCEL: Self = Self(3u32);
                pub const MB_ICONHAND: Self = Self(16u32);
                pub const MB_ICONQUESTION: Self = Self(32u32);
                pub const MB_ICONEXCLAMATION: Self = Self(48u32);
                pub const MB_ICONASTERISK: Self = Self(64u32);
                pub const MB_USERICON: Self = Self(128u32);
                pub const MB_ICONWARNING: Self = Self(48u32);
                pub const MB_ICONERROR: Self = Self(16u32);
                pub const MB_ICONINFORMATION: Self = Self(64u32);
                pub const MB_ICONSTOP: Self = Self(16u32);
                pub const MB_DEFBUTTON1: Self = Self(0u32);
                pub const MB_DEFBUTTON2: Self = Self(256u32);
                pub const MB_DEFBUTTON3: Self = Self(512u32);
                pub const MB_DEFBUTTON4: Self = Self(768u32);
                pub const MB_APPLMODAL: Self = Self(0u32);
                pub const MB_SYSTEMMODAL: Self = Self(4096u32);
                pub const MB_TASKMODAL: Self = Self(8192u32);
                pub const MB_NOFOCUS: Self = Self(32768u32);
                pub const MB_SETFOREGROUND: Self = Self(65536u32);
                pub const MB_DEFAULT_DESKTOP_ONLY: Self = Self(131072u32);
                pub const MB_TOPMOST: Self = Self(262144u32);
                pub const MB_RIGHT: Self = Self(524288u32);
                pub const MB_RTLREADING: Self = Self(1048576u32);
                pub const MB_SERVICE_NOTIFICATION: Self = Self(2097152u32);
                pub const MB_SERVICE_NOTIFICATION_NT3X: Self = Self(262144u32);
                pub const MB_TYPEMASK: Self = Self(15u32);
                pub const MB_ICONMASK: Self = Self(240u32);
                pub const MB_DEFMASK: Self = Self(3840u32);
                pub const MB_MODEMASK: Self = Self(12288u32);
                pub const MB_MISCMASK: Self = Self(49152u32);
            }
            impl ::std::convert::From<u32> for MESSAGEBOX_STYLE {
                fn from(value: u32) -> Self {
                    Self(value)
                }
            }
            unsafe impl ::windows::Abi for MESSAGEBOX_STYLE {
                type Abi = Self;
            }
            impl ::std::ops::BitOr for MESSAGEBOX_STYLE {
                type Output = Self;
                fn bitor(self, rhs: Self) -> Self {
                    Self(self.0 | rhs.0)
                }
            }
            impl ::std::ops::BitAnd for MESSAGEBOX_STYLE {
                type Output = Self;
                fn bitand(self, rhs: Self) -> Self {
                    Self(self.0 & rhs.0)
                }
            }
            impl ::std::ops::BitOrAssign for MESSAGEBOX_STYLE {
                fn bitor_assign(&mut self, rhs: Self) {
                    self.0.bitor_assign(rhs.0)
                }
            }
            impl ::std::ops::BitAndAssign for MESSAGEBOX_STYLE {
                fn bitand_assign(&mut self, rhs: Self) {
                    self.0.bitand_assign(rhs.0)
                }
            }
            pub unsafe fn MessageBoxA<'a>(
                hwnd: impl ::windows::IntoParam<'a, HWND>,
                lptext: impl ::windows::IntoParam<'a, super::SystemServices::PSTR>,
                lpcaption: impl ::windows::IntoParam<'a, super::SystemServices::PSTR>,
                utype: MESSAGEBOX_STYLE,
            ) -> MESSAGEBOX_RESULT {
                #[link(name = "USER32")]
                extern "system" {
                    pub fn MessageBoxA(
                        hwnd: HWND,
                        lptext: super::SystemServices::PSTR,
                        lpcaption: super::SystemServices::PSTR,
                        utype: MESSAGEBOX_STYLE,
                    ) -> MESSAGEBOX_RESULT;
                }
                MessageBoxA(
                    hwnd.into_param().abi(),
                    lptext.into_param().abi(),
                    lpcaption.into_param().abi(),
                    ::std::mem::transmute(utype),
                )
            }
        }
    }
}
