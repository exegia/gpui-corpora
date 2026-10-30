#[allow(unused_imports)]
use super::*;

/// Dropping the receiver cancels the producer.
pub fn spawn_token_stream<S, E>(cx: &App, stream: S) -> (Task<()>, Receiver<String>)
where
    S: Stream<Item = Result<String, E>> + Send + 'static,
    E: std::fmt::Display + Send + 'static,
{
    let (tx, rx) = flume::bounded::<String>(CHANNEL_CAPACITY);

    let pump = async move {
        use futures_util::StreamExt as _;
        let mut stream = std::pin::pin!(stream);
        while let Some(item) = stream.next().await {
            match item {
                Ok(token) => {
                    if tx.send_async(token).await.is_err() {
                        // Receiver dropped — cancel the stream.
                        break;
                    }
                }
                Err(err) => {
                    tracing::warn!(
                        target: "corpora::streaming",
                        error = %err,
                        "token stream produced an error; terminating"
                    );
                    // Surface the error as a final chunk so callers that render
                    // inline still see something, then stop.
                    let _ = tx.send_async(err.to_string()).await;
                    break;
                }
            }
        }
        // tx drops at end of scope → receiver sees channel close.
    };

    (producer_task(cx, pump), rx)
}
