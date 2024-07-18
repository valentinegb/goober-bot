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

#[cfg(debug_assertions)]
pub(crate) const A_FLOOF_LOAD: &str = "<a:afloofLoad:1263605189995266058>";
#[cfg(not(debug_assertions))]
pub(crate) const A_FLOOF_LOAD: &str = "<:afloofLoad:1263609041179906059>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF: &str = "<:floof:1263605435785810104>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF: &str = "<:floof:1263609061539315722>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_ANGRY: &str = "<:floofAngry:1263605462927016077>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_ANGRY: &str = "<:floofAngry:1263609077661962331>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_BLEP: &str = "<:floofBlep:1263605485488308295>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_BLEP: &str = "<:floofBlep:1263609094791495724>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_CAT: &str = "<:floofCat:1263605506593915053>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_CAT: &str = "<:floofCat:1263609111581560862>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_COOL: &str = "<:floofCool:1263605526160474112>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_COOL: &str = "<:floofCool:1263609129683910761>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_CRY: &str = "<:floofCry:1263605545852600393>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_CRY: &str = "<:floofCry:1263609147824410684>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_DROOL: &str = "<:floofDrool:1263605564035039323>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_DROOL: &str = "<:floofDrool:1263609166875066438>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_HAPPY: &str = "<:floofHappy:1263605580380110890>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_HAPPY: &str = "<:floofHappy:1263609184415383613>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_HEART: &str = "<:floofHeart:1263605598524539001>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_HEART: &str = "<:floofHeart:1263609201431675002>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_INNOCENT: &str = "<:floofInnocent:1263605617034006619>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_INNOCENT: &str = "<:floofInnocent:1263609220725608519>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_LOAD: &str = "<:floofLoad:1263605636411949118>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_LOAD: &str = "<:floofLoad:1263609237762871336>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_LOL: &str = "<:floofLol:1263605657886654495>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_LOL: &str = "<:floofLol:1263609255647510668>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_LURK: &str = "<:floofLurk:1263605681420894258>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_LURK: &str = "<:floofLurk:1263609272818729082>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_MISCHIEF: &str = "<:floofMischief:1263605706733650041>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_MISCHIEF: &str = "<:floofMischief:1263609299838697552>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_MUG: &str = "<:floofMug:1263605736517271634>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_MUG: &str = "<:floofMug:1263609319555993792>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_NERVOUS: &str = "<:floofNervous:1263605768700301386>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_NERVOUS: &str = "<:floofNervous:1263609339013501008>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_NOM: &str = "<:floofNom:1263605793710800897>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_NOM: &str = "<:floofNom:1263609382801903666>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_OWO: &str = "<:floofOwO:1263605821338816583>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_OWO: &str = "<:floofOwO:1263609400732418089>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_PAT: &str = "<:floofPat:1263605857300643951>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_PAT: &str = "<:floofPat:1263609418214543371>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_PEEK: &str = "<:floofPeek:1263605875906449570>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_PEEK: &str = "<:floofPeek:1263609437726179479>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_PLEAD: &str = "<:floofPlead:1263605895930052668>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_PLEAD: &str = "<:floofPlead:1263609456760062072>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_SAD: &str = "<:floofSad:1263605923188965428>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_SAD: &str = "<:floofSad:1263609478440288317>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_SCARED: &str = "<:floofScared:1263605944839831614>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_SCARED: &str = "<:floofScared:1263609529820647544>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_SMUG: &str = "<:floofSmug:1263605963877912577>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_SMUG: &str = "<:floofSmug:1263609552356773971>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_TEEHEE: &str = "<:floofTeehee:1263605984115560478>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_TEEHEE: &str = "<:floofTeehee:1263609577815933061>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_TIRED: &str = "<:floofTired:1263606003082199131>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_TIRED: &str = "<:floofTired:1263609597382496308>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_WHAT: &str = "<:floofWhat:1263606024892321945>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_WHAT: &str = "<:floofWhat:1263609615036190821>";
#[cfg(debug_assertions)]
pub(crate) const FLOOF_WOOZY: &str = "<:floofWoozy:1263606042840010752>";
#[cfg(not(debug_assertions))]
pub(crate) const FLOOF_WOOZY: &str = "<:floofWoozy:1263609632333762592>";
