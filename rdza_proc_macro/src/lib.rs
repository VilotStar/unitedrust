use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "jak" => "as",
        "przerwa" => "break",
        "stały" => "const",
        "kontynuować" => "continue",
        "skrzynia" => "crate",
        "inny" => "else",
        "wyliczenie" => "enum",
        "zewnętrzny" => "extern",
        "fałsz" => "false",
        "funkcjonować" => "fn",
        "dle" => "for",
        "jeśli" => "if",
        "narzędzie" => "impl",
        "w" => "in",
        "pozwalać" => "let",
        "pętla" => "loop",
        "mecz" => "match",
        "modulacja" => "mod",
        "przenosić" => "move",
        "zmienny" => "mut",
        "publiczny" => "pub",
        "ref" => "ref",
        "powrót" => "return",
        "ja" => "self",
        "struktura" => "struct",
        "super" => "super",
        "cecha" => "trait",
        "prawda" => "true",
        "rodzaj" => "type",
        "niebezpieczny" => "unsafe",
        "używać" => "use",
        "gdzie" => "where",
        "podczas" => "while",
        "asynchroniczny" => "async",
        "oczekiwaćt" => "await",
        "dynamiczny" => "dyn",
        "abstrakcyjny" => "abstract",
        "zostać" => "become",
        "skrzynka" => "box",
        "zrobić" => "do",
        "finał" => "final",
        "makro" => "macro",
        "outrepasser" => "override",
        "prywatny" => "priv",
        "typ" => "typeof",
        "niewielkości" => "unsized",
        "wirtualny" => "virtual",
        "dawać" => "yield",
        "próbować" => "try",
        "zasady_makro" => "macro_rules",
        "unia" => "union",
        "statyczny" => "static",
        "bool" => "bool",
        "unumer" => "usize",
        "inumer" => "isize",
        "zwęglać" => "char",
        "strunowy" => "str",
        "Skrzynka" => "Box",
        "Ol" => "Rc",
        "Aol" => "Arc",
        "Szpilka" => "Pin",
        "NiebezpiecznyKomórka" => "UnsafeCell",
        "FantomDane" => "PhantomData",
        "Dereferencja" => "Deref",
        "DereferencjaZmienny" => "DerefMut",
        "Upuszczać" => "Drop",
        "Kopiuj" => "Copy",
        "Klon" => "Clone",
        "Wysłać" => "Send",
        "Synchronizuj" => "Sync",
        "Zakończenie" => "Termination",
        "Nieszpilka" => "Unpin",
        "Rozmiar" => "Sized",
        "Strunowy" => "String",
        "panika" => "panic",
        "zawierać_ciąg" => "include_str",
        "podział" => "split",
        "podzielić_o_godz" => "split_at",
        "zbierać" => "collect",
        "czerpać" => "derive",
        "Wektor" => "Vec",
        "naciskać" => "push",
        "odwijaćsię" => "unwrap",
        "domyślny" => "default",
        "Domyślny" => "Default",
        "Ja" => "Self",
        "do" => "into",
        "wydrukować" => "print",
        "wydrukowaćln" => "println",
        "suma" => "sum",
        "analizować" => "parse",
        "Nic" => "None",
        "Niektóre" => "Some",
        "Opcja" => "Option",
        "Błąd" => "Err",
        "BłądVal" => "Error",
        "OK" => "Ok",
        "HaszyszMapa" => "HashMap",
        "Wynik" => "Result",
        "nieosiągalny_kod" => "unreachable_code",
        "z" => "from",
        "jako_ref" => "as_ref",
        "dostawać" => "get",
        "umożliwić" => "allow",
        "pobierz_lub_wstaw_za_pomocą" => "get_or_insert_with",
        "główny" => "main",
        "oczekiwać" => "expect",
        "rozszerzyć" => "extend",
        "makro_proc" => "proc_macro",
        "wyliczać" => "enumerate",
        "długość" => "len",
        "znaki" => "chars",
        "filtr" => "filter",
        "zawiera" => "contains",
        "nty" => "nth",
        "jest_małe_litery" => "is_ascii_lowercase",
        "jest_duże_litery" => "is_ascii_uppercase",
        "do_sznurka" => "to_string",
        "mapa" => "map",
        "jako_zmienny_plasterek" => "as_mut_slice",
        "jako_plasterek" => "as_slice",
        "sortować" => "sort",
        "odwracać" => "reverse",
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
pub fn rdza(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
