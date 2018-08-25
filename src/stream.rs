use responses::Update;
use error::Error;
use std::collections::VecDeque;
use futures::Future;
use futures::Async;
use futures::Stream;
use std::cmp::max;

pub(crate) struct UpdatesStream<Fut, Sender> {
    pub bot_api_client: Sender,
    pub buffer: VecDeque<Update>,
    pub executing_request: Fut,
    pub is_canceled: bool,
}

impl<Fut, Sender> Stream for UpdatesStream<Fut, Sender>
    where Fut: Future<Item=Vec<Update>, Error=Error>,
          Sender: FnMut(i64) -> Fut {
    type Item = Update;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        if self.is_canceled {
            return Ok(Async::Ready(None));
        }
        if let Some(update) = self.buffer.pop_front() {
            return Ok(Async::Ready(Some(update)));
        }
        match self.executing_request.poll() {
            Ok(Async::NotReady) =>
                Ok(Async::NotReady),

            Ok(Async::Ready(updates)) => {
                let mut max_id = -1;
                for update in updates {
                    max_id = max(update.id, max_id);
                    self.buffer.push_back(update)
                }
                self.executing_request = (self.bot_api_client)(max_id + 1);
                self.poll()
            }

            Err(err) =>
                Err(err)
        }
    }
}

impl<Fut, Sender> Drop for UpdatesStream<Fut, Sender> {
    fn drop(&mut self) {
        self.is_canceled = true;
    }
}