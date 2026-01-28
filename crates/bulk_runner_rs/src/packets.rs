use bulk_runner_bots::{Bot, BotStatus, BotStatusReady};

impl From<Dispatchable> for Vec<(Bot, String)> {
    fn from(dispatchable: Dispatchable) -> Self {
        dispatchable
            .bots
            .into_iter()
            .map(|packet| (packet.bot, packet.process_name))
            .collect()
    }
}

pub struct Packet {
    pub bot:          Bot,
    pub process_name: String,
}

pub struct Dispatchable {
    pub bots: Vec<Packet>,
}

impl Packet {
    #[must_use]
    pub fn new(bot: Bot, process_name: String) -> Self {
        Packet { bot, process_name }
    }
}

impl<I> From<I> for Dispatchable
where
    I: IntoIterator<Item = Packet>,
{
    fn from(iter: I) -> Self {
        Dispatchable {
            bots: iter.into_iter().collect(),
        }
    }
}

impl FromIterator<Packet> for Dispatchable {
    fn from_iter<T: IntoIterator<Item = Packet>>(iter: T) -> Self {
        let bots = iter
            .into_iter()
            .map(|mut packet| {
                if packet.bot.is_logged_out() {
                    packet.bot.status = BotStatus::Ready(BotStatusReady::Idle);
                    packet
                } else {
                    packet
                }
            })
            .collect::<Vec<Packet>>();

        Dispatchable { bots }
    }
}
