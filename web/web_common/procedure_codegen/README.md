# Procedure Codegen

Crate providing code generation for handling the creation of procedure and procedure builder enumerations.

## Tasks

- [ ] Generate enumeration of insertable builders.
- [ ] Create a trait mapping a procedure template to its concrete procedure.
- [ ] Create trait for procedure builder iterator, which most likely needs to go in `web_common_traits` crate.
- [ ] Trackables that are defined as `shared` by using the `shared_procedure_template_trackables` table need to be initialized in the procedure builders if they have already been inserted in previous procedures.
