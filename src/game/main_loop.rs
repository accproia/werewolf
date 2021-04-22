use futures::StreamExt;
use telegram_bot::*;
use log::*;


pub struct MainLoop {
    token: String
}


impl MainLoop {

    pub fn new(token: &str) -> MainLoop {
        MainLoop{token: String::from(token)}
    }

    pub async fn run(&self) -> Result<(), Error> {
        
        info!("thread started.");

        let api = Api::new(&self.token[..]);
        let mut stream = api.stream();
    
        while let Some(update) = stream.next().await {
            
            info!("{:?}", update);
            let update = update?;
    
            if let UpdateKind::Message(message) = update.kind {
                if let MessageKind::Text{ref data, ref entities} = message.kind {
                    info!("{}", data);

                    for (i, it) in entities.iter().enumerate() {
                        let start = it.offset as usize;
                        let end = (it.offset + it.length) as usize;
                        match &it.kind {
                            MessageEntityKind::Mention => info!("Mention {}: {}", i, &data[start..end]),
                            MessageEntityKind::Hashtag => info!("Hashtag {}: {}", i, &data[start..end]),
                            MessageEntityKind::BotCommand => info!("BotCommand {}: {}", i, &data[start..end]),
                            MessageEntityKind::Url => info!("Url {}: {}", i, &data[start..end]),
                            MessageEntityKind::Email => info!("Email {}: {}", i, &data[start..end]),
                            MessageEntityKind::Bold => info!("Bold {}: {}", i, &data[start..end]),
                            MessageEntityKind::Italic => info!("Italic {}: {}", i, &data[start..end]),
                            MessageEntityKind::Code => info!("Code {}: {}", i, &data[start..end]),
                            MessageEntityKind::Pre => info!("Pre {}: {}", i, &data[start..end]),
                            MessageEntityKind::TextLink(text) => info!("TextLink {}: {}; text: {}", i, &data[start..end], text),
                            MessageEntityKind::TextMention(user) => info!("TextMention {}: {}; user: {}", i, &data[start..end], user.first_name),
                            _ => info!("Unknown")
                        }
                    } 
                }
            }
        }

        info!("thread is about to end.");

        Ok(())
    }

}