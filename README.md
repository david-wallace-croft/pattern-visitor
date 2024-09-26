# Pattern Visitor

- Example Rust source code demonstrating the pattern "Visitor"

## Usage

- cd pattern-visitor/
- cargo -q run

## Examples

- visitor_0
  - Demonstrates using a Visitor to add functionality to an Element
- visitor_1
  - Demonstrates expanding the supported Element types using a generic
- visitor_2
  - Demonstrates how to extend a Visitor to visit a new external Element
- visitor_3
  - Compares the Visitor pattern to using Mixin
- visitor_4
  - A Visitor which mutates an Element using values from the other Elements
  - Removes an Element from a shared reference to Elements during mutation
- visitor_5
  - A Visitor which mutates an Element using values from the other Elements
  - Accesses the data from Elements before mutating the Element
- visitor_6
  - A Visitor which mutates an Element using values from the other Elements
  - Uses interior mutability for each Element in a shared reference to Elements

## History

- Initial release: 2024-09-08
