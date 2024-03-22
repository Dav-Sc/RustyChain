A realtional Database for Genelology

Newdata type: gdate

.generate(ID(i32), NAME(std), SURNAME(std), MotherID(i32), FatherID(i32), DOB, DOD, INFO(std))



ID, NAME, SURNAME
MotherID, FatherID
DOB, DOD, INFO


VM where you can manage jsons
- Add, remove, edit, search, sort, filter, etc.

commands:
> createdb <name>
> removedb <name>
> listdb
> find <name>
> printdb <name>
> add <var name> <json>
> remove <var name> <json>
- if remove ID remove all children

REST API server
- can send and recieve data