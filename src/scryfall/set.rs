use serde::{Deserialize, Serialize};

/// A Set object represents a group of related Magic cards. All Card objects on
/// Scryfall belong to exactly one set.
///
/// Due to Magic’s long and complicated history, Scryfall includes many
/// un-official sets as a way to group promotional or outlier cards together.
/// Such sets will likely have a code that begins with `p` or `t`, such as
/// `pcel` or `tori`.
///
/// Official sets always have a three-letter set code, such as `zen`.
#[derive(Serialize, Deserialize)]
struct ScryfallSetObject {
    /// A unique ID for this set on Scryfall that will not change.
    id:              String,
    /// The unique three to five-letter code for this set.
    code:            String,
    /// The unique code for this set on MTGO, which may differ from the regular
    /// code.
    mtgo_code:       Option<String>,
    /// This set’s ID on [TCGplayer’s API](https://docs.tcgplayer.com/docs), also known as the groupId
    tcgplayer_id:    Option<u32>,
    /// The English name of the set.
    name:            String,
    /// A computer-readable classification for this set.
    set_type:        ScryfallSetType,
    /// The date the set was released or the first card was printed in the set
    /// (in GMT-8 Pacific time).
    released_at:     Option<String>,
    /// The block code for this set, if any.
    block_code:      Option<String>,
    /// The block or group name code for this set, if any.
    block:           Option<String>,
    /// The set code for the parent set, if any. promo and token sets often have
    /// a parent set.
    parent_set_code: Option<String>,
    /// The number of cards in this set.
    card_count:      u32,
    /// The denominator for the set’s printed collector numbers.
    printed_size:    Option<u32>,
    /// True if this set was only released in a video game.
    digital:         bool,
    /// True if this set contains only foil cards.
    foil_only:       bool,
    /// True if this set contains only nonfoil cards.
    nonfoil_only:    bool,
    /// A link to this set’s permapage on Scryfall’s website.
    scryfall_uri:    String,
    /// A link to this set object on Scryfall’s API.
    uri:             String,
    /// A URI to an SVG file for this set’s icon on Scryfall’s CDN. Hotlinking
    /// this image isn’t recommended, because it may change slightly over time.
    /// You should download it and use it locally for your particular user
    /// interface needs.
    icon_svg_uri:    String,
    /// A Scryfall API URI that you can request to begin paginating over the
    /// cards in this set.
    search_uri:      String
}

/// Scryfall provides an overall categorization for each Set in the `set_type`
/// property.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ScryfallSetType {
    /// A yearly Magic core set (Tenth Edition, etc)
    Core,
    /// A rotational expansion set in a block (Zendikar, etc)
    Expansion,
    /// A reprint set that contains no new cards (Modern Masters, etc)
    Masters,
    /// An Arena set designed for Alchemy
    Alchemy,
    /// Masterpiece Series premium foil cards
    Masterpiece,
    /// A Commander-oriented gift set
    Arsenal,
    /// From the Vault gift sets
    FromTheVault,
    /// Spellbook series gift sets
    Spellbook,
    /// Premium Deck Series decks
    PremiumDeck,
    /// Duel Decks
    DuelDeck,
    /// Special draft sets, like Conspiracy and Battlebond
    DraftInnovation,
    /// Magic Online treasure chest prize sets
    TreasureChest,
    /// Commander preconstructed decks
    Commander,
    /// Planechase sets
    Planechase,
    /// Archenemy sets
    Archenemy,
    /// Vanguard card sets
    Vanguard,
    /// A funny un-set or set with funny promos (Unglued, Happy Holidays, etc)
    Funny,
    /// A starter/introductory set (Portal, etc)
    Starter,
    /// A gift box set
    Box,
    /// A set that contains purely promotional cards
    Promo,
    /// A set made up of tokens and emblems.
    Token,
    /// A set made up of gold-bordered, oversize, or trophy cards that are not
    /// legal
    Memorabilia,
    /// A set that contains minigame card inserts from booster packs
    Minigame
}
