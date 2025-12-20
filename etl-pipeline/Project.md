Project : Build a data pipeline that can read from any source(String, File , Dummy API ) and write to any destination(Console, File), without changing the runner code

**Constraint**
- Use Traits to define the interface 
- Use Box<dyn Trait> to store a list of mixed processor. 