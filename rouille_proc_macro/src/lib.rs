use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "comme" => "as",
        "casser" => "break",
        "constante" => "const",
        "continuer" => "continue",
        "caisse" => "crate",
        "autre" => "else",
        "énumération" => "enum",
        "externe" => "extern",
        "faux" => "false",
        "fonction" => "fn",
        "pour" => "for",
        "si" => "if",
        "implémentation" => "impl",
        "dans" => "in",
        "laisser" => "let",
        "boucle" => "loop",
        "correspondre" => "match",
        "modulation" => "mod",
        "déplacer" => "move",
        "mutable" => "mut",
        "publique" => "pub",
        "réf" => "ref",
        "retour" => "return",
        "soi" => "self",
        "statique" => "static",
        "structure" => "struct",
        "super" => "super",
        "trait" => "trait",
        "vrai" => "true",
        "taper" => "type",
        "dangereux" => "unsafe",
        "utiliser" => "use",
        "où" => "where",
        "instant" => "while",
        "asynchrone" => "async",
        "attendre" => "await",
        "dynamique" => "dyn",
        "abstrait" => "abstract",
        "devenir" => "become",
        "'boîte" => "box",
        "faire" => "do",
        "final" => "final",
        "macro" => "macro",
        "outrepasser" => "override",
        "privé" => "priv",
        "taper" => "typeof",
        "pastaille" => "unsized",
        "virtuel" => "virtual",
        "rendement" => "yield",
        "essayer" => "try",
        "règles_de_macro!" => "macro_rules!",
        "syndicat" => "union",
        "'statique" => "'static",
        "booléen" => "bool",
        "unombre" => "usize",
        "inombre" => "isize",
        "carboniser" => "char",
        "&chaîne" => "&str",
        "Boîte<" => "Box<",
        "Rc<" => "Rc<",
        "Arc<" => "Arc<",
        "Épingle<" => "Pin<",
        "DangereuxCellule<" => "UnsafeCell<",
        "FantômeDonnées<" => "PhantomData<",
        "Déréférencement" => "Deref",
        "DéréférencementMutable" => "DerefMut",
        "Baisse" => "Drop",
        "Copie" => "Copy",
        "Cloner" => "Clone",
        "Envoyer" => "Send",
        "Synchroniser" => "Sync",
        "Résiliation" => "Termination",
        "Pasépingle" => "Unpin",
        "Dimensionné" => "Sized",
        "Chaîne" => "String",
        "panique" => "panic",
        "inclure_une_chaîne" => "include_str",
        "diviser" => "split",
        "divisé_à" => "split_at",
        "collecter" => "collect",
        "dériver" => "derive",
        "Vecteur" => "Vec",
        "pousser" => "push",
        "déballer" => "unwrap",
        "défaut" => "default",
        "Défaut" => "Default",
        "Soi" => "Self",
        "danst" => "into",
        "imprimer" => "print",
        "imprimerln" => "println",
        "somme" => "sum",
        "analyser" => "parse",
        "Aucun" => "None",
        "Quelques" => "Some",
        "Option" => "Option",
        "Erreur" => "Err",
        "ErreurVal" => "Error",
        "D'accord" => "Ok",
        "HacherCarte" => "HashMap",
        "Résultat" => "Result",
        "code_inaccessible" => "unreachable_code",
        "depuis" => "from",
        "comme_réf" => "as_ref",
        "obtenir" => "get",
        "permettre" => "allow",
        "obtenir_ou_insérer_avec" => "get_or_insert_with",
        "principal" => "main",
        "attendre" => "expect",
        "étendre" => "extend",
        "macro_de_procédure" => "proc_macro",
        "énumérer" => "enumerate",
        "longueur" => "len",
        "caractères" => "chars",
        "filtre" => "filter",
        "contient" => "contains",
        "nième" => "nth",
        "est_minuscule" => "is_ascii_lowercase",
        "est_majuscule" => "is_ascii_uppercase",
        "enchaîner" => "to_string",
        "carte" => "map",
        "comme_tranche_mutable" => "as_mut_slice",
        "comme_tranche" => "as_slice",
        "trier" => "sort",
        "inverse" => "reverse",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rouille(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
