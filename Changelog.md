# Changelog
## 0.5.0-alpha.4
- update to sauron `0.24.0`

## 0.5.0-alpha.3
- remove local paths
- update to sauron `0.23.0`

## 0.5.0-alpha.2
-  re-export `sauron::Node`

## 0.5.0-alpha.1
- Expose the with-dom feature in svgbob, this is to allow usage of the generated node readily usable as is in sauron projects, provided they uses the same version of sauron library
- Make functions that return Node<()> into a return generics Node<MSG> to allow usage of Node<MSG> generics in sauron apps
- Fix the behaviour of merging line and polygon tags, the direction of the arrow is now correct and only dealing with line that is in the same direction of the arrow marker
- Merging line with arrow and heading with opposite direction is not yet dealth

## 0.5.0-alpha.0
- rewrite core architecture to support shape tagging and styling
