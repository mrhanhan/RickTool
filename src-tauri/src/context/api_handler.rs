use super::api_manager::{ApiHandler, CallContext};
use serde::{Serialize, de::{DeserializeOwned}};


#[allow(unused)]
pub(crate) type JsonApiHandler<D, R> = dyn Fn(&mut CallContext, D) -> Result<R, String> + Sync + Send;
#[allow(unused)]
pub(crate) type JsonApiHandlerRef<D, R> = dyn Fn(&mut CallContext, &mut D) -> Result<R, String> + Sync + Send;

#[allow(unused)]
pub(crate) fn api_json<D: Sized + DeserializeOwned + 'static, R: Serialize + 'static>(handler: Box<JsonApiHandler<D, R>>) -> Box<ApiHandler>
{
    Box::new(move |_ctx| {
        let result = serde_json::from_str::<D>(&_ctx.data.as_str());
        if let Ok(_data) = result {
            let result = handler(_ctx, _data);
            match result {
                Ok(_resp) => {
                    match serde_json::to_string(&_resp) {
                        Ok(_data) => {
                            Ok(_data)
                        },
                        Err(err) => {
                            Err(err.to_string())
                        }
                    }
                },
                Err(_reason) => {Err(_reason)},
            }
        } else {
            return Err("JSON_序列化失败".into());
        }
       
    })
}

#[allow(unused)]
pub(crate) fn api_json_ref<D: Sized + DeserializeOwned + 'static, R: Serialize + 'static>(handler: Box<JsonApiHandlerRef<D, R>>) -> Box<ApiHandler>
{
    Box::new(move |_ctx| {
        let result = serde_json::from_str::<D>(&_ctx.data.as_str());
        if let Ok(mut _data) = result {
            let result = handler(_ctx, &mut _data);
            match result {
                Ok(_resp) => {
                    match serde_json::to_string(&_resp) {
                        Ok(_data) => {
                            Ok(_data)
                        },
                        Err(err) => {
                            Err(err.to_string())
                        }
                    }
                },
                Err(_reason) => {Err(_reason)},
            }
        } else {
            return Err("JSON_序列化失败".into());
        }
       
    })
}
