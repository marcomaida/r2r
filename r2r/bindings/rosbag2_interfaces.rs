  pub mod srv {
#[allow(non_snake_case)]
    pub mod GetRate {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosbag2_interfaces__srv__GetRate()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosbag2_interfaces__srv__GetRate_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__GetRate_Request() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__GetRate_Request {

                unsafe { rosbag2_interfaces__srv__GetRate_Request__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__GetRate_Request) -> () {

                unsafe { rosbag2_interfaces__srv__GetRate_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub rate: f64,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosbag2_interfaces__srv__GetRate_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__GetRate_Response() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__GetRate_Response {

                unsafe { rosbag2_interfaces__srv__GetRate_Response__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__GetRate_Response) -> () {

                unsafe { rosbag2_interfaces__srv__GetRate_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
rate: msg.rate,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.rate = self.rate;
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod IsPaused {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosbag2_interfaces__srv__IsPaused()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosbag2_interfaces__srv__IsPaused_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__IsPaused_Request() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__IsPaused_Request {

                unsafe { rosbag2_interfaces__srv__IsPaused_Request__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__IsPaused_Request) -> () {

                unsafe { rosbag2_interfaces__srv__IsPaused_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub paused: bool,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosbag2_interfaces__srv__IsPaused_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__IsPaused_Response() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__IsPaused_Response {

                unsafe { rosbag2_interfaces__srv__IsPaused_Response__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__IsPaused_Response) -> () {

                unsafe { rosbag2_interfaces__srv__IsPaused_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
paused: msg.paused,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.paused = self.paused;
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod Pause {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosbag2_interfaces__srv__Pause()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosbag2_interfaces__srv__Pause_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__Pause_Request() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__Pause_Request {

                unsafe { rosbag2_interfaces__srv__Pause_Request__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__Pause_Request) -> () {

                unsafe { rosbag2_interfaces__srv__Pause_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              
                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosbag2_interfaces__srv__Pause_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__Pause_Response() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__Pause_Response {

                unsafe { rosbag2_interfaces__srv__Pause_Response__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__Pause_Response) -> () {

                unsafe { rosbag2_interfaces__srv__Pause_Response__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Response {
  Response {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod PlayNext {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosbag2_interfaces__srv__PlayNext()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosbag2_interfaces__srv__PlayNext_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__PlayNext_Request() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__PlayNext_Request {

                unsafe { rosbag2_interfaces__srv__PlayNext_Request__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__PlayNext_Request) -> () {

                unsafe { rosbag2_interfaces__srv__PlayNext_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub success: bool,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosbag2_interfaces__srv__PlayNext_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__PlayNext_Response() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__PlayNext_Response {

                unsafe { rosbag2_interfaces__srv__PlayNext_Response__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__PlayNext_Response) -> () {

                unsafe { rosbag2_interfaces__srv__PlayNext_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod Resume {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosbag2_interfaces__srv__Resume()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosbag2_interfaces__srv__Resume_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__Resume_Request() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__Resume_Request {

                unsafe { rosbag2_interfaces__srv__Resume_Request__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__Resume_Request) -> () {

                unsafe { rosbag2_interfaces__srv__Resume_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              
                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosbag2_interfaces__srv__Resume_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__Resume_Response() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__Resume_Response {

                unsafe { rosbag2_interfaces__srv__Resume_Response__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__Resume_Response) -> () {

                unsafe { rosbag2_interfaces__srv__Resume_Response__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Response {
  Response {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod Seek {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosbag2_interfaces__srv__Seek()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub time: builtin_interfaces::msg::Time,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosbag2_interfaces__srv__Seek_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__Seek_Request() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__Seek_Request {

                unsafe { rosbag2_interfaces__srv__Seek_Request__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__Seek_Request) -> () {

                unsafe { rosbag2_interfaces__srv__Seek_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
time: builtin_interfaces::msg::Time::from_native(&msg.time),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.time.copy_to_native(&mut msg.time);
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub success: bool,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosbag2_interfaces__srv__Seek_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__Seek_Response() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__Seek_Response {

                unsafe { rosbag2_interfaces__srv__Seek_Response__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__Seek_Response) -> () {

                unsafe { rosbag2_interfaces__srv__Seek_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod SetRate {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosbag2_interfaces__srv__SetRate()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub rate: f64,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosbag2_interfaces__srv__SetRate_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__SetRate_Request() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__SetRate_Request {

                unsafe { rosbag2_interfaces__srv__SetRate_Request__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__SetRate_Request) -> () {

                unsafe { rosbag2_interfaces__srv__SetRate_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
rate: msg.rate,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.rate = self.rate;
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub success: bool,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosbag2_interfaces__srv__SetRate_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__SetRate_Response() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__SetRate_Response {

                unsafe { rosbag2_interfaces__srv__SetRate_Response__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__SetRate_Response) -> () {

                unsafe { rosbag2_interfaces__srv__SetRate_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod TogglePaused {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosbag2_interfaces__srv__TogglePaused()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosbag2_interfaces__srv__TogglePaused_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__TogglePaused_Request() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__TogglePaused_Request {

                unsafe { rosbag2_interfaces__srv__TogglePaused_Request__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__TogglePaused_Request) -> () {

                unsafe { rosbag2_interfaces__srv__TogglePaused_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              
                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosbag2_interfaces__srv__TogglePaused_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbag2_interfaces__srv__TogglePaused_Response() }
            }

            fn create_msg() -> *mut rosbag2_interfaces__srv__TogglePaused_Response {

                unsafe { rosbag2_interfaces__srv__TogglePaused_Response__create() }

            }

            fn destroy_msg(msg: *mut rosbag2_interfaces__srv__TogglePaused_Response) -> () {

                unsafe { rosbag2_interfaces__srv__TogglePaused_Response__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Response {
  Response {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
  }
