#[test]
fn rect1() {
    let bob = r#"
    +----------+
    |          |
    +----------+
    "#;

    let expected = r#"<svg xmlns="http://www.w3.org/2000/svg" width="136" height="80">
    <style>line, path, circle,rect,polygon {
                          stroke: black;
                          stroke-width: 2;
                          stroke-opacity: 1;
                          fill-opacity: 1;
                          stroke-linecap: round;
                          stroke-linejoin: miter;
                        }

                    text {
                        fill: black;
                        }
                        rect.backdrop{
                            stroke: none;
                            fill: white;
                        }
                        .broken{
                            stroke-dasharray: 8;
                        }
                        .filled{
                            fill: black;
                        }
                        .bg_filled{
                            fill: white;
                        }
                        .nofill{
                            fill: white;
                        }

                        text {
                         font-family: monospace;
                         font-size: 14px;
                        }

                        .end_marked_arrow{
                            marker-end: url(#arrow);
                         }
                        .start_marked_arrow{
                            marker-start: url(#arrow);
                         }

                        .end_marked_diamond{
                            marker-end: url(#diamond);
                         }
                        .start_marked_diamond{
                            marker-start: url(#diamond);
                         }

                        .end_marked_circle{
                            marker-end: url(#circle);
                         }
                        .start_marked_circle{
                            marker-start: url(#circle);
                         }

                        .end_marked_open_circle{
                            marker-end: url(#open_circle);
                         }
                        .start_marked_open_circle{
                            marker-start: url(#open_circle);
                         }

                        .end_marked_big_open_circle{
                            marker-end: url(#big_open_circle);
                         }
                        .start_marked_big_open_circle{
                            marker-start: url(#big_open_circle);
                         }

                         
                        </style>
    <defs>
        <marker id="arrow" viewBox="-2 -2 8 8" refX="4" refY="2" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
            <polygon points="0,0 0,4 4,2 0,0"></polygon>
        </marker>
        <marker id="diamond" viewBox="-2 -2 8 8" refX="4" refY="2" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
            <polygon points="0,2 2,0 4,2 2,4 0,2"></polygon>
        </marker>
        <marker id="circle" viewBox="0 0 8 8" refX="4" refY="4" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
            <circle cx="4" cy="4" r="2" class="filled"></circle>
        </marker>
        <marker id="open_circle" viewBox="0 0 8 8" refX="4" refY="4" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
            <circle cx="4" cy="4" r="2" class="bg_filled"></circle>
        </marker>
        <marker id="big_open_circle" viewBox="0 0 8 8" refX="4" refY="4" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
            <circle cx="4" cy="4" r="3" class="bg_filled"></circle>
        </marker>
    </defs>
    <rect class="backdrop" x="0" y="0" width="136" height="80"></rect>
    <rect x="36" y="24" width="88" height="32" class="solid nofill" rx="0"></rect>
</svg>"#;

    let svg = svgbob::to_svg(bob);
    println!("{}", svg);
    assert_eq!(expected, svg);
}
