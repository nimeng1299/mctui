use std::thread;


/// 用于获取单个网页的内容
pub struct SingleDownloader {
    event: SingleDownloaderEvent,
    tx: std::sync::mpsc::Sender<Result<String, String>>,
    rx: std::sync::mpsc::Receiver<Result<String, String>>,
    result: String,
    error: String,
}

impl SingleDownloader{
    pub fn get_state(&mut self) -> SingleDownloaderEvent{
        match self.event {
            SingleDownloaderEvent::None => SingleDownloaderEvent::None,
            SingleDownloaderEvent::Progress => {
                match self.rx.try_recv(){
                    Ok(res) => {
                        match res {
                            Ok(content) => {
                                self.result = content;
                                self.event = SingleDownloaderEvent::Finished;
                                SingleDownloaderEvent::Finished
                            },
                            Err(err) => {
                                self.error = format!("{:?}", err);
                                self.event = SingleDownloaderEvent::Failed;
                                SingleDownloaderEvent::Failed
                            }
                        }
                    },
                    Err(_) => SingleDownloaderEvent::Progress,
                }
            },
            SingleDownloaderEvent::Finished => SingleDownloaderEvent::Finished,
            SingleDownloaderEvent::Failed => SingleDownloaderEvent::Failed,
        }
    }

    // 开始获取下载的内容，之前下载的所有内容会被清空
    pub fn download(&mut self, url: String){
        self.event = SingleDownloaderEvent::Progress;
        self.result.clear();
        self.error.clear();

        let tx = self.tx.clone();
        thread::spawn(move || {
           match attohttpc::get(url).send() {
                Ok(res) => match res.text() {
                    Ok(content) => tx.send(Ok(content)),
                    Err(err) => tx.send(Err(err.to_string())),
                },
                Err(err) => tx.send(Err(err.to_string())),
           }
            
        });
    }

    pub fn set_none(&mut self){
        self.event = SingleDownloaderEvent::None;
        self.result.clear();
        self.error.clear();
    }

    pub fn get_data(&self) -> String{
        self.result.clone()
    }

    pub fn get_error(&self) -> String{
        self.error.clone()
    }
}

impl Default for SingleDownloader {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<Result<String, String>>();

        Self {
            event: SingleDownloaderEvent::None,
            tx,
            rx,
            result: String::new(),
            error: String::new(),
        }
    }
}

#[derive(Default, PartialEq)]
pub enum SingleDownloaderEvent {
    #[default]
    None,
    Progress,
    Finished,
    Failed,
}