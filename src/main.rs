use clap::Parser;
use kafka::consumer::{Consumer, FetchOffset};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Config {
    #[clap(short = 'b', long, env)]
    pub brokers: Vec<String>,
    #[clap(short = 't', long, env)]
    pub topic: String,
    #[clap(short = 'g', long, env, default_value = "my-consumer-group")]
    pub group: String,
}

fn main() {
    let config = Config::parse();
    println!("{:?}", config);

    let mut consumer = Consumer::from_hosts(config.brokers)
        .with_topic(config.topic)
        .with_group(config.group)
        .with_fallback_offset(FetchOffset::Earliest)
        .create()
        .unwrap();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let partition = ms.partition();
                let offset = m.offset();
                let k = String::from_utf8_lossy(m.key);
                let v = String::from_utf8_lossy(m.value);
                println!("{}/{}\tK:{} V:{}", partition, offset, k, v);
            }
            consumer.consume_messageset(ms).unwrap();
        }
        consumer.commit_consumed().unwrap();
    }
}
