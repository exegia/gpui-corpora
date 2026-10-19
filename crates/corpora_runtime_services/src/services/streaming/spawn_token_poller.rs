#[allow(unused_imports)]
use super::*;

/// The returned [`Task`] cancels polling when dropped or replaced.
pub fn spawn_token_poller<T, F>(
    rx: Receiver<String>,
    weak: WeakEntity<T>,
    cx: &mut Context<T>,
    mut on_token: F,
) -> Task<()>
where
    T: 'static,
    F: FnMut(&mut T, &str) + Send + 'static,
{
    cx.spawn(async move |_this: WeakEntity<T>, cx: &mut AsyncApp| {
        while let Ok(token) = rx.recv_async().await {
            let _ = cx.update(|cx: &mut App| {
                if let Some(this) = weak.upgrade() {
                    let _ = this.update(cx, |this, cx| {
                        on_token(this, &token);
                        cx.notify();
                    });
                }
            });
        }
        // Channel closed = stream finished. Notify the view so it can flip its
        // "streaming" flag off if it hasn't already.
        let _ = cx.update(|cx: &mut App| {
            if let Some(this) = weak.upgrade() {
                let _ = this.update(cx, |_, cx| {
                    cx.notify();
                });
            }
        });
    })
}
