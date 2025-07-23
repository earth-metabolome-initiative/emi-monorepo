#[derive(Debug, Clone, Copy, PartialEq)]
enum LinkShape {
    ArrowHead,
    Open,
    Dotted,
    Thick,
    Invisible,
    CircleEdge,
    CrossEdge,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LinkType {
    WithText,
    NoText,
}
