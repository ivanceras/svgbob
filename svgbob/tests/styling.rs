#[test]

fn test_styling() {
    let bob = r#"
.----------.   .----------.
|{w} A     |-->|{w} B     |
'----------'   '----------'

.----------.   .----------.
|{w} A     |<--|{w} B     |
'----------'   '----------'

.----------.   .----------.
|{w} A     |<->|{w} B     |
'----------'   '----------'

# Legend:
w = {
 fill: #abadb0;
}
"#;

    let svg = svgbob::to_svg_string_compressed(bob);

    println!("{}", &svg);

    let _expected = r#"<svg xmlns="http://www.w3.org/2000/svg" width="224" height="208"><style>line, path, circle,rect,polygon{stroke:black;stroke-width:2;stroke-opacity:1;fill-opacity:1;stroke-linecap:round;stroke-linejoin:miter;}text{font-family:monospace;font-size:14px;}rect.backdrop{stroke:none;fill:white;}.broken{stroke-dasharray:8;}.filled{fill:black;}.bg_filled{fill:white;}.nofill{fill:white;}.end_marked_arrow{marker-end:url(#arrow);}.start_marked_arrow{marker-start:url(#arrow);}.end_marked_diamond{marker-end:url(#diamond);}.start_marked_diamond{marker-start:url(#diamond);}.end_marked_circle{marker-end:url(#circle);}.start_marked_circle{marker-start:url(#circle);}.end_marked_open_circle{marker-end:url(#open_circle);}.start_marked_open_circle{marker-start:url(#open_circle);}.end_marked_big_open_circle{marker-end:url(#big_open_circle);}.start_marked_big_open_circle{marker-start:url(#big_open_circle);}.w{
 fill: #abadb0;
 }</style><defs><marker id="arrow" viewBox="-2 -2 8 8" refX="4" refY="2" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><polygon points="0,0 0,4 4,2 0,0"></polygon></marker><marker id="diamond" viewBox="-2 -2 8 8" refX="4" refY="2" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><polygon points="0,2 2,0 4,2 2,4 0,2"></polygon></marker><marker id="circle" viewBox="0 0 8 8" refX="4" refY="4" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><circle cx="4" cy="4" r="2" class="filled"></circle></marker><marker id="open_circle" viewBox="0 0 8 8" refX="4" refY="4" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><circle cx="4" cy="4" r="2" class="bg_filled"></circle></marker><marker id="big_open_circle" viewBox="0 0 8 8" refX="4" refY="4" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><circle cx="4" cy="4" r="3" class="bg_filled"></circle></marker></defs><rect class="backdrop" x="0" y="0" width="224" height="208"></rect><rect x="124" y="24" width="88" height="32" class="solid nofill w" rx="4"></rect><text x="162" y="44" >B</text><rect x="4" y="88" width="88" height="32" class="solid nofill w" rx="4"></rect><text x="42" y="108" >A</text><rect x="4" y="152" width="88" height="32" class="solid nofill w" rx="4"></rect><text x="42" y="172" >A</text><rect x="124" y="152" width="88" height="32" class="solid nofill w" rx="4"></rect><text x="162" y="172" >B</text><text x="10" y="44" >{w}</text><text x="42" y="44" >A</text><text x="130" y="108" >{w}</text><text x="162" y="108" >B</text><g><path d="M 8,24 A 4,4 0,0,0 4,28" class="nofill"></path><line x1="4" y1="28" x2="4" y2="52" class="solid"></line><line x1="8" y1="24" x2="88" y2="24" class="solid"></line><path d="M 88,24 A 4,4 0,0,1 92,28" class="nofill"></path><line x1="92" y1="28" x2="92" y2="52" class="solid"></line><line x1="92" y1="40" x2="112" y2="40" class="solid"></line><polygon points="112,36 120,40 112,44" class="filled"></polygon><path d="M 4,52 A 4,4 0,0,0 8,56" class="nofill"></path><line x1="8" y1="56" x2="88" y2="56" class="solid"></line><path d="M 92,52 A 4,4 0,0,1 88,56" class="nofill"></path></g><g><path d="M 128,88 A 4,4 0,0,0 124,92" class="nofill"></path><line x1="124" y1="92" x2="124" y2="116" class="solid"></line><line x1="128" y1="88" x2="208" y2="88" class="solid"></line><path d="M 208,88 A 4,4 0,0,1 212,92" class="nofill"></path><line x1="212" y1="92" x2="212" y2="116" class="solid"></line><path d="M 124,116 A 4,4 0,0,0 128,120" class="nofill"></path><line x1="128" y1="120" x2="208" y2="120" class="solid"></line><path d="M 212,116 A 4,4 0,0,1 208,120" class="nofill"></path><polygon points="104,100 96,104 104,108" class="filled"></polygon><line x1="104" y1="104" x2="124" y2="104" class="solid"></line></g><g><polygon points="104,164 96,168 104,172" class="filled"></polygon><line x1="104" y1="168" x2="112" y2="168" class="solid"></line><polygon points="112,164 120,168 112,172" class="filled"></polygon></g></svg>"#;
}
