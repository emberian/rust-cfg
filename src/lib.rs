use std::collections::VecMap;
/// A Context-Free Grammar
///
/// A context-free grammar consists of a set of terminals (called tokens), a set of non-terminals
/// (called variables), a set of productions, and a specific variable used as the start symbol.  A
/// production is a mapping from a variable to a possibly empty list of symbols (a symbol is either
/// a token or a variable).
///
/// A symbol is internally represented as a u64, differentiating between tokens and variables by
/// assuming token values are less than or equal to a fixed value, `last_token`, which represents
/// the last token. A string "representing" a symbol in some way can be retrieved with
/// `cfg.name(sym)`.

pub type Rule = Vec<u64>;

pub struct Cfg {
    rules: VecMap<Vec<Rule>>,
    symbol_map: VecMap<String>,
    start: u64,
    last_token: u64
}

impl Cfg {
    pub fn new(last_token: u64) -> Cfg {
        Cfg {
            rules: VecMap::new(),
            symbol_map: VecMap::new(),
            start: -1,
            last_token: last_token
        }
    }

    /// Create a complete `Cfg` from its constituent pieces.
    ///
    /// Returns `None` if the start symbol is a token, or if there are variables mentioned with no
    /// corresponding rule.
    pub fn from_pieces(rules: VecMap<Vec<Rule>>,
                       symbol_map: VecMap<String>,
                       start: u64, last_token: u64) -> Option<Cfg> {
        if start <= last_token {
            return None
        }

        for (_, all_rules) in rules.iter() {
            for rule in all_rules.iter() {
                for &symbol in rule.iter() {
                    if symbol > last_token && rules.get(&(symbol as uint)).is_none() {
                        return None
                    }
                }
            }
        }

        Some(Cfg {
            rules: rules,
            symbol_map: symbol_map,
            start: start,
            last_token: last_token
        })
    }

    pub fn get_start(&self) -> u64 {
        self.start
    }

    /// Get the name of a symbol.
    pub fn name(&self, symbol: u64) -> Option<&str> {
        self.symbol_map.get(&(symbol as uint)).map(|x| x.as_slice())
    }

    /// Add a rule to the grammar.
    pub fn add_rule(&mut self, variable: u64, body: Rule) {
        assert!(variable > self.last_token);
        match self.rules.get_mut(&(variable as uint)) {
            Some(v) => {
                v.push(body);
                return;
            },
            None => { }
        }
        self.rules.insert(variable as uint, vec![body]);
    }

    /// Set the name of a symbol, returning the old name if any.
    pub fn set_name(&mut self, symbol: u64, name: String) -> Option<String> {
        self.symbol_map.insert(symbol as uint, name)
    }

    pub fn get_rules(&self, variable: u64) -> Option<&[Rule]> {
        assert!(variable > self.last_token);
        self.rules.get(&(variable as uint)).map(|x| x.as_slice())
    }
}
