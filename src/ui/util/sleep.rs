/// A hacked-together sleep function that works in both server and web contexts.
pub async fn sleep(duration: std::time::Duration) {
    #[cfg(feature = "ssr")]
    {
        tokio::time::sleep(duration).await
    }
    #[cfg(feature = "web")]
    {
        use wasm_bindgen::JsCast;
        let (send, recv) = futures::channel::oneshot::channel();
        let closure = wasm_bindgen::closure::Closure::once(|| {
            let _ = send.send(());
        });
        let _timeout = web_sys::window()
            .expect("should be able to get window")
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                duration.as_millis() as i32,
            );

        closure.forget();
        let _ = recv.await;
    }
}
