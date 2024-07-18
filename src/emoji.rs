// Goober Bot, Discord bot
// Copyright (C) 2024  Valentine Briese
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![allow(dead_code)]

// Hey,
// https://cdn.discordapp.com/emojis/<id>.webp?size=48&quality=lossless
// ;)

// in debug builds, uses emojis from Goober Bot Dev app
// in release builds, uses emojis from Goober Bot app

/// ![](https://cdn.discordapp.com/emojis/1263605189995266058.gif?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const A_FLOOF_LOAD: &str = "<a:afloofLoad:1263605189995266058>";
/// ![](https://cdn.discordapp.com/emojis/1263609041179906059.gif?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const A_FLOOF_LOAD: &str = "<:afloofLoad:1263609041179906059>";
/// ![](https://cdn.discordapp.com/emojis/1263605435785810104.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF: &str = "<:floof:1263605435785810104>";
/// ![](https://cdn.discordapp.com/emojis/1263609061539315722.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF: &str = "<:floof:1263609061539315722>";
/// ![](https://cdn.discordapp.com/emojis/1263605462927016077.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_ANGRY: &str = "<:floofAngry:1263605462927016077>";
/// ![](https://cdn.discordapp.com/emojis/1263609077661962331.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_ANGRY: &str = "<:floofAngry:1263609077661962331>";
/// ![](https://cdn.discordapp.com/emojis/1263605485488308295.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_BLEP: &str = "<:floofBlep:1263605485488308295>";
/// ![](https://cdn.discordapp.com/emojis/1263609094791495724.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_BLEP: &str = "<:floofBlep:1263609094791495724>";
/// ![](https://cdn.discordapp.com/emojis/1263605506593915053.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_CAT: &str = "<:floofCat:1263605506593915053>";
/// ![](https://cdn.discordapp.com/emojis/1263609111581560862.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_CAT: &str = "<:floofCat:1263609111581560862>";
/// ![](https://cdn.discordapp.com/emojis/1263605526160474112.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_COOL: &str = "<:floofCool:1263605526160474112>";
/// ![](https://cdn.discordapp.com/emojis/1263609129683910761.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_COOL: &str = "<:floofCool:1263609129683910761>";
/// ![](https://cdn.discordapp.com/emojis/1263605545852600393.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_CRY: &str = "<:floofCry:1263605545852600393>";
/// ![](https://cdn.discordapp.com/emojis/1263609147824410684.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_CRY: &str = "<:floofCry:1263609147824410684>";
/// ![](https://cdn.discordapp.com/emojis/1263605564035039323.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_DROOL: &str = "<:floofDrool:1263605564035039323>";
/// ![](https://cdn.discordapp.com/emojis/1263609166875066438.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_DROOL: &str = "<:floofDrool:1263609166875066438>";
/// ![](https://cdn.discordapp.com/emojis/1263605580380110890.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_HAPPY: &str = "<:floofHappy:1263605580380110890>";
/// ![](https://cdn.discordapp.com/emojis/1263609184415383613.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_HAPPY: &str = "<:floofHappy:1263609184415383613>";
/// ![](https://cdn.discordapp.com/emojis/1263605598524539001.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_HEART: &str = "<:floofHeart:1263605598524539001>";
/// ![](https://cdn.discordapp.com/emojis/1263609201431675002.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_HEART: &str = "<:floofHeart:1263609201431675002>";
/// ![](https://cdn.discordapp.com/emojis/1263605617034006619.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_INNOCENT: &str = "<:floofInnocent:1263605617034006619>";
/// ![](https://cdn.discordapp.com/emojis/1263609220725608519.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_INNOCENT: &str = "<:floofInnocent:1263609220725608519>";
/// ![](https://cdn.discordapp.com/emojis/1263605636411949118.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_LOAD: &str = "<:floofLoad:1263605636411949118>";
/// ![](https://cdn.discordapp.com/emojis/1263609237762871336.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_LOAD: &str = "<:floofLoad:1263609237762871336>";
/// ![](https://cdn.discordapp.com/emojis/1263605657886654495.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_LOL: &str = "<:floofLol:1263605657886654495>";
/// ![](https://cdn.discordapp.com/emojis/1263609255647510668.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_LOL: &str = "<:floofLol:1263609255647510668>";
/// ![](https://cdn.discordapp.com/emojis/1263605681420894258.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_LURK: &str = "<:floofLurk:1263605681420894258>";
/// ![](https://cdn.discordapp.com/emojis/1263609272818729082.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_LURK: &str = "<:floofLurk:1263609272818729082>";
/// ![](https://cdn.discordapp.com/emojis/1263605706733650041.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_MISCHIEF: &str = "<:floofMischief:1263605706733650041>";
/// ![](https://cdn.discordapp.com/emojis/1263609299838697552.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_MISCHIEF: &str = "<:floofMischief:1263609299838697552>";
/// ![](https://cdn.discordapp.com/emojis/1263605736517271634.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_MUG: &str = "<:floofMug:1263605736517271634>";
/// ![](https://cdn.discordapp.com/emojis/1263609319555993792.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_MUG: &str = "<:floofMug:1263609319555993792>";
/// ![](https://cdn.discordapp.com/emojis/1263605768700301386.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_NERVOUS: &str = "<:floofNervous:1263605768700301386>";
/// ![](https://cdn.discordapp.com/emojis/1263609339013501008.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_NERVOUS: &str = "<:floofNervous:1263609339013501008>";
/// ![](https://cdn.discordapp.com/emojis/1263605793710800897.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_NOM: &str = "<:floofNom:1263605793710800897>";
/// ![](https://cdn.discordapp.com/emojis/1263609382801903666.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_NOM: &str = "<:floofNom:1263609382801903666>";
/// ![](https://cdn.discordapp.com/emojis/1263605821338816583.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_OWO: &str = "<:floofOwO:1263605821338816583>";
/// ![](https://cdn.discordapp.com/emojis/1263609400732418089.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_OWO: &str = "<:floofOwO:1263609400732418089>";
/// ![](https://cdn.discordapp.com/emojis/1263605857300643951.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_PAT: &str = "<:floofPat:1263605857300643951>";
/// ![](https://cdn.discordapp.com/emojis/1263609418214543371.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_PAT: &str = "<:floofPat:1263609418214543371>";
/// ![](https://cdn.discordapp.com/emojis/1263605875906449570.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_PEEK: &str = "<:floofPeek:1263605875906449570>";
/// ![](https://cdn.discordapp.com/emojis/1263609437726179479.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_PEEK: &str = "<:floofPeek:1263609437726179479>";
/// ![](https://cdn.discordapp.com/emojis/1263605895930052668.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_PLEAD: &str = "<:floofPlead:1263605895930052668>";
/// ![](https://cdn.discordapp.com/emojis/1263609456760062072.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_PLEAD: &str = "<:floofPlead:1263609456760062072>";
/// ![](https://cdn.discordapp.com/emojis/1263605923188965428.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_SAD: &str = "<:floofSad:1263605923188965428>";
/// ![](https://cdn.discordapp.com/emojis/1263609478440288317.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_SAD: &str = "<:floofSad:1263609478440288317>";
/// ![](https://cdn.discordapp.com/emojis/1263605944839831614.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_SCARED: &str = "<:floofScared:1263605944839831614>";
/// ![](https://cdn.discordapp.com/emojis/1263609529820647544.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_SCARED: &str = "<:floofScared:1263609529820647544>";
/// ![](https://cdn.discordapp.com/emojis/1263605963877912577.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_SMUG: &str = "<:floofSmug:1263605963877912577>";
/// ![](https://cdn.discordapp.com/emojis/1263609552356773971.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_SMUG: &str = "<:floofSmug:1263609552356773971>";
/// ![](https://cdn.discordapp.com/emojis/1263605984115560478.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_TEEHEE: &str = "<:floofTeehee:1263605984115560478>";
/// ![](https://cdn.discordapp.com/emojis/1263609577815933061.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_TEEHEE: &str = "<:floofTeehee:1263609577815933061>";
/// ![](https://cdn.discordapp.com/emojis/1263606003082199131.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_TIRED: &str = "<:floofTired:1263606003082199131>";
/// ![](https://cdn.discordapp.com/emojis/1263609597382496308.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_TIRED: &str = "<:floofTired:1263609597382496308>";
/// ![](https://cdn.discordapp.com/emojis/1263606024892321945.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_WHAT: &str = "<:floofWhat:1263606024892321945>";
/// ![](https://cdn.discordapp.com/emojis/1263609615036190821.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_WHAT: &str = "<:floofWhat:1263609615036190821>";
/// ![](https://cdn.discordapp.com/emojis/1263606042840010752.webp?&quality=lossless)
#[cfg(debug_assertions)]
pub(crate) const FLOOF_WOOZY: &str = "<:floofWoozy:1263606042840010752>";
/// ![](https://cdn.discordapp.com/emojis/1263609632333762592.webp?&quality=lossless)
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_WOOZY: &str = "<:floofWoozy:1263609632333762592>";
