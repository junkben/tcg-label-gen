use crate::scryfall::ScryfallSetType;

pub struct SetTypeFilter {
    /// A yearly Magic core set (Tenth Edition, etc)
    core:             bool,
    /// A rotational expansion set in a block (Zendikar, etc)
    expansion:        bool,
    /// A reprint set that contains no new cards (Modern Masters, etc)
    masters:          bool,
    /// An Arena set designed for Alchemy
    alchemy:          bool,
    /// Masterpiece Series premium foil cards
    masterpiece:      bool,
    /// A Commander-oriented gift set
    arsenal:          bool,
    /// From the Vault gift sets
    from_the_vault:   bool,
    /// Spellbook series gift sets
    spellbook:        bool,
    /// Premium Deck Series decks
    premium_deck:     bool,
    /// Duel Decks
    duel_deck:        bool,
    /// Special draft sets, like Conspiracy and Battlebond
    draft_innovation: bool,
    /// Magic Online treasure chest prize sets
    treasure_chest:   bool,
    /// Commander preconstructed decks
    commander:        bool,
    /// Planechase sets
    planechase:       bool,
    /// Archenemy sets
    archenemy:        bool,
    /// Vanguard card sets
    vanguard:         bool,
    /// A funny un-set or set with funny promos (Unglued, Happy Holidays, etc)
    funny:            bool,
    /// A starter/introductory set (Portal, etc)
    starter:          bool,
    /// A gift box set
    gbox:             bool,
    /// A set that contains purely promotional cards
    promo:            bool,
    /// A set made up of tokens and emblems.
    token:            bool,
    /// A set made up of gold-bordered, oversize, or trophy cards that are not
    /// legal
    memorabilia:      bool,
    /// A set that contains minigame card inserts from booster packs
    minigame:         bool
}

impl Default for SetTypeFilter {
    fn default() -> Self {
        Self {
            core:             true,
            expansion:        true,
            masters:          true,
            alchemy:          false,
            masterpiece:      false,
            arsenal:          false,
            from_the_vault:   true,
            spellbook:        false,
            premium_deck:     true,
            duel_deck:        true,
            draft_innovation: true,
            treasure_chest:   false,
            commander:        true,
            planechase:       true,
            archenemy:        true,
            vanguard:         false,
            funny:            true,
            starter:          true,
            gbox:             true,
            promo:            false,
            token:            false,
            memorabilia:      false,
            minigame:         false
        }
    }
}

impl SetTypeFilter {
    pub fn allowed(&self, set_type: &ScryfallSetType) -> bool {
        use ScryfallSetType::*;
        match set_type {
            Core => self.core,
            Expansion => self.expansion,
            Masters => self.masters,
            Alchemy => self.alchemy,
            Masterpiece => self.masterpiece,
            Arsenal => self.arsenal,
            FromTheVault => self.from_the_vault,
            Spellbook => self.spellbook,
            PremiumDeck => self.premium_deck,
            DuelDeck => self.duel_deck,
            DraftInnovation => self.draft_innovation,
            TreasureChest => self.treasure_chest,
            Commander => self.commander,
            Planechase => self.planechase,
            Archenemy => self.archenemy,
            Vanguard => self.vanguard,
            Funny => self.funny,
            Starter => self.starter,
            Box => self.gbox,
            Promo => self.promo,
            Token => self.token,
            Memorabilia => self.memorabilia,
            Minigame => self.minigame
        }
    }
}
