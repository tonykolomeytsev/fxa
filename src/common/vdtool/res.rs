use lazy_static::lazy_static;
use std::collections::{BTreeMap, BTreeSet};

pub const SVG_DEFS: &str = "defs";
pub const SVG_USE: &str = "use";
pub const SVG_HREF: &str = "href";
pub const SVG_XLINK_HREF: &str = "xlink:href";

pub const SVG_POLYGON: &str = "polygon";
pub const SVG_POLYLINE: &str = "polyline";
pub const SVG_RECT: &str = "rect";
pub const SVG_CIRCLE: &str = "circle";
pub const SVG_LINE: &str = "line";
pub const SVG_PATH: &str = "path";
pub const SVG_ELLIPSE: &str = "ellipse";
pub const SVG_GROUP: &str = "g";
pub const SVG_STYLE: &str = "style";
pub const SVG_DISPLAY: &str = "display";
pub const SVG_CLIP_PATH_ELEMENT: &str = "clipPath";

pub const SVG_D: &str = "d";
pub const SVG_STROKE: &str = "stroke";
pub const SVG_STROKE_OPACITY: &str = "stroke-opacity";
pub const SVG_STROKE_LINEJOIN: &str = "stroke-linejoin";
pub const SVG_STROKE_LINECAP: &str = "stroke-linecap";
pub const SVG_STROKE_WIDTH: &str = "stroke-width";
pub const SVG_FILL: &str = "fill";
pub const SVG_FILL_OPACITY: &str = "fill-opacity";
pub const SVG_FILL_RULE: &str = "fill-rule";
pub const SVG_OPACITY: &str = "opacity";
pub const SVG_CLIP: &str = "clip";
pub const SVG_CLIP_PATH: &str = "clip-path";
pub const SVG_CLIP_RULE: &str = "clip-rule";
pub const SVG_MASK: &str = "mask";
pub const SVG_POINTS: &str = "points";

lazy_static! {
    /// Set of all SVG nodes that we don't support. Categorized by the types.
    pub static ref UNSUPPORTED_SVG_NODES: BTreeSet<&'static str> = BTreeSet::from([
        // Animation elements.
        "animate",
        "animateColor",
        "animateMotion",
        "animateTransform",
        "mpath",
        "set",
        // Container elements.
        "a",
        "glyph",
        "marker",
        "missing-glyph",
        "pattern",
        "switch",
        "symbol",
        // Filter primitive elements.
        "feBlend",
        "feColorMatrix",
        "feComponentTransfer",
        "feComposite",
        "feConvolveMatrix",
        "feDiffuseLighting",
        "feDisplacementMap",
        "feFlood",
        "feFuncA",
        "feFuncB",
        "feFuncG",
        "feFuncR",
        "feGaussianBlur",
        "feImage",
        "feMerge",
        "feMergeNode",
        "feMorphology",
        "feOffset",
        "feSpecularLighting",
        "feTile",
        "feTurbulence",
        // Font elements.
        "font",
        "font-face",
        "font-face-format",
        "font-face-name",
        "font-face-src",
        "font-face-uri",
        "hkern",
        "vkern",
        // Gradient elements.
        "stop",
        // Graphics elements.
        "ellipse",
        "image",
        "text",
        // Light source elements.
        "feDistantLight",
        "fePointLight",
        "feSpotLight",
        // Structural elements.
        "symbol",
        // Text content elements.
        "altGlyph",
        "altGlyphDef",
        "altGlyphItem",
        "glyph",
        "glyphRef",
        "textPath",
        "text",
        "tref",
        "tspan",
        // Text content child elements.
        "altGlyph",
        "textPath",
        "tref",
        "tspan",
        // Uncategorized elements.
        "color-profile",
        "cursor",
        "filter",
        "foreignObject",
        "script",
        "view"
    ]);

    pub static ref PRESENTATION_MAP: BTreeMap<&'static str, &'static str> = BTreeMap::from([
        (SVG_CLIP, "android:clip"),
        // TODO: (SVG_CLIP_RULE, ""), // Treated individually.
        (SVG_FILL, "android:fillColor"),
        (SVG_FILL_RULE, "android:fillType"),
        (SVG_FILL_OPACITY, "android:fillAlpha"),
        // TODO: (SVG_OPACITY, ""), // Treated individually.
        (SVG_STROKE, "android:strokeColor"),
        (SVG_STROKE_OPACITY, "android:strokeAlpha"),
        (SVG_STROKE_LINEJOIN, "android:strokeLineJoin"),
        (SVG_STROKE_LINECAP, "android:strokeLineCap"),
        (SVG_STROKE_WIDTH, "android:strokeWidth"),
    ]);

    pub static ref GRADIENT_MAP: BTreeMap<&'static str, &'static str> = BTreeMap::from([
        ("x1", "android:startX"),
        ("y1", "android:startY"),
        ("x2", "android:endX"),
        ("y2", "android:endY"),
        ("cx", "android:centerX"),
        ("cy", "android:centerY"),
        ("r", "android:gradientRadius"),
        ("spreadMethod", "android:tileMode"),
        ("gradientUnits", ""),
        ("gradientTransform", ""),
        ("gradientType", "android:type"),
    ]);

    pub static ref COLOR_MAP: BTreeMap<&'static str, &'static str> = BTreeMap::from([
        ("aliceblue", "#f0f8ff"),
        ("antiquewhite", "#faebd7"),
        ("aqua", "#00ffff"),
        ("aquamarine", "#7fffd4"),
        ("azure", "#f0ffff"),
        ("beige", "#f5f5dc"),
        ("bisque", "#ffe4c4"),
        ("black", "#000000"),
        ("blanchedalmond", "#ffebcd"),
        ("blue", "#0000ff"),
        ("blueviolet", "#8a2be2"),
        ("brown", "#a52a2a"),
        ("burlywood", "#deb887"),
        ("cadetblue", "#5f9ea0"),
        ("chartreuse", "#7fff00"),
        ("chocolate", "#d2691e"),
        ("coral", "#ff7f50"),
        ("cornflowerblue", "#6495ed"),
        ("cornsilk", "#fff8dc"),
        ("crimson", "#dc143c"),
        ("cyan", "#00ffff"),
        ("darkblue", "#00008b"),
        ("darkcyan", "#008b8b"),
        ("darkgoldenrod", "#b8860b"),
        ("darkgray", "#a9a9a9"),
        ("darkgrey", "#a9a9a9"),
        ("darkgreen", "#006400"),
        ("darkkhaki", "#bdb76b"),
        ("darkmagenta", "#8b008b"),
        ("darkolivegreen", "#556b2f"),
        ("darkorange", "#ff8c00"),
        ("darkorchid", "#9932cc"),
        ("darkred", "#8b0000"),
        ("darksalmon", "#e9967a"),
        ("darkseagreen", "#8fbc8f"),
        ("darkslateblue", "#483d8b"),
        ("darkslategray", "#2f4f4f"),
        ("darkslategrey", "#2f4f4f"),
        ("darkturquoise", "#00ced1"),
        ("darkviolet", "#9400d3"),
        ("deeppink", "#ff1493"),
        ("deepskyblue", "#00bfff"),
        ("dimgray", "#696969"),
        ("dimgrey", "#696969"),
        ("dodgerblue", "#1e90ff"),
        ("firebrick", "#b22222"),
        ("floralwhite", "#fffaf0"),
        ("forestgreen", "#228b22"),
        ("fuchsia", "#ff00ff"),
        ("gainsboro", "#dcdcdc"),
        ("ghostwhite", "#f8f8ff"),
        ("gold", "#ffd700"),
        ("goldenrod", "#daa520"),
        ("gray", "#808080"),
        ("grey", "#808080"),
        ("green", "#008000"),
        ("greenyellow", "#adff2f"),
        ("honeydew", "#f0fff0"),
        ("hotpink", "#ff69b4"),
        ("indianred", "#cd5c5c"),
        ("indigo", "#4b0082"),
        ("ivory", "#fffff0"),
        ("khaki", "#f0e68c"),
        ("lavender", "#e6e6fa"),
        ("lavenderblush", "#fff0f5"),
        ("lawngreen", "#7cfc00"),
        ("lemonchiffon", "#fffacd"),
        ("lightblue", "#add8e6"),
        ("lightcoral", "#f08080"),
        ("lightcyan", "#e0ffff"),
        ("lightgoldenrodyellow", "#fafad2"),
        ("lightgray", "#d3d3d3"),
        ("lightgrey", "#d3d3d3"),
        ("lightgreen", "#90ee90"),
        ("lightpink", "#ffb6c1"),
        ("lightsalmon", "#ffa07a"),
        ("lightseagreen", "#20b2aa"),
        ("lightskyblue", "#87cefa"),
        ("lightslategray", "#778899"),
        ("lightslategrey", "#778899"),
        ("lightsteelblue", "#b0c4de"),
        ("lightyellow", "#ffffe0"),
        ("lime", "#00ff00"),
        ("limegreen", "#32cd32"),
        ("linen", "#faf0e6"),
        ("magenta", "#ff00ff"),
        ("maroon", "#800000"),
        ("mediumaquamarine", "#66cdaa"),
        ("mediumblue", "#0000cd"),
        ("mediumorchid", "#ba55d3"),
        ("mediumpurple", "#9370db"),
        ("mediumseagreen", "#3cb371"),
        ("mediumslateblue", "#7b68ee"),
        ("mediumspringgreen", "#00fa9a"),
        ("mediumturquoise", "#48d1cc"),
        ("mediumvioletred", "#c71585"),
        ("midnightblue", "#191970"),
        ("mintcream", "#f5fffa"),
        ("mistyrose", "#ffe4e1"),
        ("moccasin", "#ffe4b5"),
        ("navajowhite", "#ffdead"),
        ("navy", "#000080"),
        ("oldlace", "#fdf5e6"),
        ("olive", "#808000"),
        ("olivedrab", "#6b8e23"),
        ("orange", "#ffa500"),
        ("orangered", "#ff4500"),
        ("orchid", "#da70d6"),
        ("palegoldenrod", "#eee8aa"),
        ("palegreen", "#98fb98"),
        ("paleturquoise", "#afeeee"),
        ("palevioletred", "#db7093"),
        ("papayawhip", "#ffefd5"),
        ("peachpuff", "#ffdab9"),
        ("peru", "#cd853f"),
        ("pink", "#ffc0cb"),
        ("plum", "#dda0dd"),
        ("powderblue", "#b0e0e6"),
        ("purple", "#800080"),
        ("rebeccapurple", "#663399"),
        ("red", "#ff0000"),
        ("rosybrown", "#bc8f8f"),
        ("royalblue", "#4169e1"),
        ("saddlebrown", "#8b4513"),
        ("salmon", "#fa8072"),
        ("sandybrown", "#f4a460"),
        ("seagreen", "#2e8b57"),
        ("seashell", "#fff5ee"),
        ("sienna", "#a0522d"),
        ("silver", "#c0c0c0"),
        ("skyblue", "#87ceeb"),
        ("slateblue", "#6a5acd"),
        ("slategray", "#708090"),
        ("slategrey", "#708090"),
        ("snow", "#fffafa"),
        ("springgreen", "#00ff7f"),
        ("steelblue", "#4682b4"),
        ("tan", "#d2b48c"),
        ("teal", "#008080"),
        ("thistle", "#d8bfd8"),
        ("tomato", "#ff6347"),
        ("turquoise", "#40e0d0"),
        ("violet", "#ee82ee"),
        ("wheat", "#f5deb3"),
        ("white", "#ffffff"),
        ("whitesmoke", "#f5f5f5"),
        ("yellow", "#ffff00"),
        ("yellowgreen", "#9acd32"),
    ]);
}
