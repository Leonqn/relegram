use std::cmp::max;
use std::collections::VecDeque;

use futures::Async;
use futures::Future;
use futures::Stream;

use error::Error;
use responses::Update;
use std::i64;

pub struct UpdatesStream<Fut, Sender> {
    pub bot_api_client: Sender,
    pub buffer: VecDeque<Update>,
    pub executing_request: Fut,
    pub is_canceled: bool,
    pub last_id: Option<i64>,
    pub has_error: bool,
}

impl<Fut, Sender> Stream for UpdatesStream<Fut, Sender>
    where Fut: Future<Item=Vec<Update>, Error=Error>,
          Sender: FnMut(Option<i64>) -> Fut {
    type Item = Update;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        if self.is_canceled {
            return Ok(Async::Ready(None));
        }
        if let Some(update) = self.buffer.pop_front() {
            return Ok(Async::Ready(Some(update)));
        }
        if self.has_error {
            self.has_error = false;
            self.executing_request = (self.bot_api_client)(self.last_id)
        }
        match self.executing_request.poll() {
            Ok(Async::NotReady) =>
                Ok(Async::NotReady),

            Ok(Async::Ready(updates)) => {
                let last_id = self.last_id.unwrap_or(-1);
                for update in updates {
                    self.last_id = Some(max(update.id, last_id) + 1);
                    self.buffer.push_back(update)
                }
                self.executing_request = (self.bot_api_client)(self.last_id);
                self.poll()
            }
            Err(err) => {
//                match err {
//                    Error::UnexpectedResponse { .. } => self.last_id = Some(self.last_id.map(|x| x + 1).unwrap_or(-2)),
//                    _ => {}
//                }
                self.has_error = true;
                Err(err)
            }
        }
    }
}

impl<Fut, Sender> Drop for UpdatesStream<Fut, Sender> {
    fn drop(&mut self) {
        self.is_canceled = true;
    }
}