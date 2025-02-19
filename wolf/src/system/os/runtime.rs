use std::future::Future;

#[cfg(target_arch = "wasm32")]
use {
    crate::w_log,
    serde::{de::DeserializeOwned, Serialize},
    wasm_bindgen::JsValue,
    wasm_mt::WasmMt,
};

pub struct RunTime {}

impl RunTime {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn thread<F, T>(p_fn: F) -> std::thread::JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        std::thread::spawn(p_fn)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn thread<F, T>(p_fn: F)
    where
        F: FnOnce() -> T + Serialize + DeserializeOwned + 'static,
        T: Future<Output = Result<JsValue, JsValue>> + 'static,
    {
        const TRACE: &str = "RunTime::thread";
        const PKG_JS_PATH: &str = "./pkg/wolf_demo.js";

        let f = async move {
            // init the wasm web worker
            let wasm_mt_res = WasmMt::new(PKG_JS_PATH).and_init().await;
            match wasm_mt_res {
                Ok(wasm_mt) => {
                    // init a thread from web worker
                    let thread_res = wasm_mt.thread().and_init().await;
                    match thread_res {
                        Ok(t) => {
                            // execute async closure with web worker
                            let _ = t.exec_async(p_fn).await;
                        }
                        Err(e) => {
                            w_log!("{:?}. trace info: {}", e, TRACE);
                        }
                    };
                }
                Err(e) => {
                    w_log!("{:?}. trace info: {}", e, TRACE);
                }
            };
        };
        wasm_bindgen_futures::spawn_local(f);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn green_thread<F>(p_future: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        tokio::spawn(p_future)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn spawn_local<F>(f: F) -> F::Output
    where
        F: Future,
    {
        pollster::block_on(f)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn spawn_local<F>(p_future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        wasm_bindgen_futures::spawn_local(p_future);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn sleep(p_duration: std::time::Duration) {
        std::thread::sleep(p_duration);
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn async_sleep(p_duration: std::time::Duration) {
        tokio::time::sleep(p_duration).await;
    }
    #[cfg(target_arch = "wasm32")]
    pub async fn async_sleep(p_duration: std::time::Duration) {
        wasm_mt::utils::sleep(p_duration.as_millis() as u32).await;
    }
}
