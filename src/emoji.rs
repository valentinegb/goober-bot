#![allow(dead_code)]

use poise::serenity_prelude::{self, Emoji, EmojiId, GuildId, Http};

const GOOBER_BOT_DEV_SERVER: GuildId = GuildId::new(1250948547403055114);

/// ID of a floof emoji which is in the Goober Bot Dev server.
pub(crate) struct FloofEmojiId(EmojiId);

impl FloofEmojiId {
    pub(crate) const FLOOF: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427038032134155));
    pub(crate) const FLOOF_ANGRY: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427039420452946));
    pub(crate) const FLOOF_BLEP: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427040528011355));
    pub(crate) const FLOOF_CAT: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427041719062539));
    pub(crate) const FLOOF_COOL: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427042666840186));
    pub(crate) const FLOOF_CRY: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427044512337961));
    pub(crate) const FLOOF_DROOL: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427045087215617));
    pub(crate) const FLOOF_HAPPY: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427323744190617));
    pub(crate) const FLOOF_HEART: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427049017147402));
    pub(crate) const FLOOF_INNOCENT: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427325052813343));
    pub(crate) const FLOOF_LOAD: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427052053954650));
    pub(crate) const FLOOF_LOL: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427326260510776));
    pub(crate) const FLOOF_LURK: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427054897430608));
    pub(crate) const FLOOF_MISCHIEF: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427327003033644));
    pub(crate) const FLOOF_MUG: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427058437689364));
    pub(crate) const FLOOF_NERVOUS: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427328756256788));
    pub(crate) const FLOOF_NOM: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427061826555995));
    pub(crate) const FLOOF_OW_O: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427329813090367));
    pub(crate) const FLOOF_PAT: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427065723191389));
    pub(crate) const FLOOF_PEEK: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427330870050837));
    pub(crate) const FLOOF_PLEAD: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427069489680384));
    pub(crate) const FLOOF_SAD: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427332438986882));
    pub(crate) const FLOOF_SCARED: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427072144543866));
    pub(crate) const FLOOF_SMUG: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427381398831105));
    pub(crate) const FLOOF_TEEHEE: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427075902767136));
    pub(crate) const FLOOF_TIRED: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427335496634499));
    pub(crate) const FLOOF_WHAT: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427079354552421));
    pub(crate) const FLOOF_WOOZY: FloofEmojiId = FloofEmojiId(EmojiId::new(1252427382707585185));
    pub(crate) const A_FLOOF_LOAD: FloofEmojiId = FloofEmojiId(EmojiId::new(1252431363139960893));

    pub(crate) async fn to_emoji(
        self,
        http: impl AsRef<Http>,
    ) -> Result<Emoji, serenity_prelude::Error> {
        http.as_ref().get_emoji(GOOBER_BOT_DEV_SERVER, self.0).await
    }
}
