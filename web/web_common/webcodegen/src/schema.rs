use diesel::prelude::{allow_tables_to_appear_in_same_query, table};

table! {
    information_schema.tables (table_catalog, table_schema, table_name) {
        table_catalog -> Text,
        table_schema -> Text,
        table_name -> Text,
        table_type -> Text,
        self_referencing_column_name -> Nullable<Text>,
        reference_generation -> Nullable<Text>,
        user_defined_type_catalog -> Nullable<Text>,
        user_defined_type_schema -> Nullable<Text>,
        user_defined_type_name -> Nullable<Text>,
        is_insertable_into -> Text,
        is_typed -> Text,
        commit_action -> Nullable<Text>,
    }
}

table! {
    information_schema.columns (table_catalog, table_schema, table_name, column_name) {
        table_catalog -> Text,
        table_schema -> Text,
        table_name -> Text,
        column_name -> Text,
        ordinal_position -> Integer,
        column_default -> Nullable<Text>,
        #[sql_name = "is_nullable"]
        __is_nullable -> Text,
        data_type -> Text,
        character_maximum_length -> Nullable<Integer>,
        character_octet_length -> Nullable<Integer>,
        numeric_precision -> Nullable<Integer>,
        numeric_precision_radix -> Nullable<Integer>,
        numeric_scale -> Nullable<Integer>,
        datetime_precision -> Nullable<Integer>,
        interval_type -> Nullable<Text>,
        interval_precision -> Nullable<Integer>,
        character_set_catalog -> Nullable<Text>,
        character_set_schema -> Nullable<Text>,
        character_set_name -> Nullable<Text>,
        collation_catalog -> Nullable<Text>,
        collation_schema -> Nullable<Text>,
        collation_name -> Nullable<Text>,
        domain_catalog -> Nullable<Text>,
        domain_schema -> Nullable<Text>,
        domain_name -> Nullable<Text>,
        udt_catalog -> Nullable<Text>,
        udt_schema -> Nullable<Text>,
        udt_name -> Nullable<Text>,
        scope_catalog -> Nullable<Text>,
        scope_schema -> Nullable<Text>,
        scope_name -> Nullable<Text>,
        maximum_cardinality -> Nullable<Integer>,
        dtd_identifier -> Nullable<Text>,
        is_self_referencing -> Nullable<Text>,
        is_identity -> Nullable<Text>,
        identity_generation -> Nullable<Text>,
        identity_start -> Nullable<Text>,
        identity_increment -> Nullable<Text>,
        identity_maximum -> Nullable<Text>,
        identity_minimum -> Nullable<Text>,
        identity_cycle -> Nullable<Text>,
        is_generated -> Text,
        generation_expression -> Nullable<Text>,
        is_updatable -> Text,
    }
}

table! {
    information_schema.key_column_usage (table_catalog, table_schema, table_name, column_name) {
        constraint_catalog -> Text,
        constraint_schema -> Text,
        constraint_name -> Text,
        table_catalog -> Text,
        table_schema -> Text,
        table_name -> Text,
        column_name -> Text,
        ordinal_position -> Integer,
        position_in_unique_constraint -> Nullable<Integer>,
    }
}

table! {
    information_schema.table_constraints (table_catalog, table_schema, table_name, constraint_name) {
        constraint_catalog -> Text,
        constraint_schema -> Text,
        constraint_name -> Text,
        table_catalog -> Text,
        table_schema -> Text,
        table_name -> Text,
        constraint_type -> Text,
        is_deferrable -> Text,
        initially_deferred -> Text,
        enforced -> Text,
        nulls_distinct -> Nullable<Text>,
    }
}

table! {
    information_schema.referential_constraints (constraint_catalog, constraint_schema, constraint_name) {
        constraint_catalog -> Text,
        constraint_schema -> Text,
        constraint_name -> Text,
        unique_constraint_catalog -> Nullable<Text>,
        unique_constraint_schema -> Nullable<Text>,
        unique_constraint_name -> Nullable<Text>,
        match_option -> Text,
        update_rule -> Text,
        delete_rule -> Text,
    }
}

table! {
    information_schema.constraint_column_usage (table_catalog, table_schema, table_name, column_name) {
        constraint_catalog -> Text,
        constraint_schema -> Text,
        constraint_name -> Text,
        table_catalog -> Text,
        table_schema -> Text,
        table_name -> Text,
        column_name -> Text,
    }
}

table! {
    information_schema.constraint_table_usage (table_catalog, table_schema, table_name, constraint_name) {
        constraint_catalog -> Text,
        constraint_schema -> Text,
        constraint_name -> Text,
        table_catalog -> Text,
        table_schema -> Text,
        table_name -> Text,
    }
}

table! {
    information_schema.check_constraints (constraint_catalog, constraint_schema, constraint_name) {
        constraint_catalog -> Text,
        constraint_schema -> Text,
        constraint_name -> Text,
        check_clause -> Text,
    }
}

table! {
    information_schema.domain_constraints (constraint_catalog, constraint_schema, constraint_name) {
        constraint_catalog -> Text,
        constraint_schema -> Text,
        constraint_name -> Text,
        domain_catalog -> Nullable<Text>,
        domain_schema -> Nullable<Text>,
        domain_name -> Nullable<Text>,
        is_deferrable -> Text,
        initially_deferred -> Text,
    }
}

table! {
    pg_indexes (schemaname, tablename, indexname) {
        schemaname -> Text,
        tablename -> Text,
        indexname -> Text,
        tablespace -> Nullable<Text>,
        indexdef -> Text,
    }
}

table! {
    pg_index (indexrelid) {
        indexrelid -> Oid,
        indrelid -> Oid,
        indnatts -> SmallInt,
        indnkeyatts -> SmallInt,
        indisunique -> Bool,
        indnullsnotdistinct -> Bool,
        indisprimary -> Bool,
        indisexclusion -> Bool,
        indimmediate -> Bool,
        indisclustered -> Bool,
        indisvalid -> Bool,
        indcheckxmin -> Bool,
        indisready -> Bool,
        indislive -> Bool,
        indisreplident -> Bool,
        indkey -> Array<SmallInt>,
        indcollation -> Array<Oid>,
        indclass -> Array<Oid>,
        indoption -> Array<SmallInt>,
    }
}

table! {
    pg_catalog.pg_proc (oid, proname, pronamespace) {
        oid -> Oid,
        proname -> Text,
        pronamespace -> Oid,
        proowner -> Oid,
        prolang -> Oid,
        procost -> Float,
        prorows -> Float,
        provariadic -> Oid,
        prosupport -> Oid,
        prokind -> Char,
        prosecdef -> Bool,
        proleakproof -> Bool,
        proisstrict -> Bool,
        proretset -> Bool,
        provolatile -> Char,
        proparallel -> Char,
        pronargs -> SmallInt,
        pronargdefaults -> SmallInt,
        prorettype -> Oid,
        proargtypes -> Array<Oid>,
        proallargtypes -> Nullable<Array<Oid>>,
        proargmodes -> Nullable<Array<Char>>,
        proargnames -> Nullable<Array<Text>>,
        proargdefaults -> Nullable<Array<Text>>,
        protrftypes -> Nullable<Array<Oid>>,
        prosrc -> Text,
        probin -> Nullable<Bytea>,
        prosqlbody -> Nullable<Text>,
        proconfig -> Nullable<Array<Text>>,
    }
}

table! {
    pg_catalog.pg_namespace (oid) {
        oid -> Oid,
        nspname -> Text,
        nspowner -> Oid,
    }
}

table! {
    pg_catalog.pg_extension (oid) {
        oid -> Oid,
        extname -> Text,
        extowner -> Oid,
        extnamespace -> Oid,
        extrelocatable -> Bool,
        extversion -> Text,
        extconfig -> Nullable<Array<Oid>>,
        extcondition -> Nullable<Array<Text>>,
    }
}

table! {
    pg_catalog.pg_depend (classid, objid, objsubid) {
        classid -> Oid,
        objid -> Oid,
        objsubid -> Integer,
        refclassid -> Oid,
        refobjid -> Oid,
        refobjsubid -> Integer,
        deptype -> Char,
    }
}

#[cfg(not(feature = "postgres_17"))]
table! {
    pg_type (oid, typname, typnamespace) {
        oid -> Oid,
        typname -> Text,
        typnamespace -> Oid,
        typowner -> Oid,
        typlen -> SmallInt,
        typbyval -> Bool,
        typtype -> Char,
        typcategory -> Char,
        typispreferred -> Bool,
        typisdefined -> Bool,
        typdelim -> Char,
        typrelid -> Oid,
        typelem -> Oid,
        typarray -> Oid,
        typinput -> Oid,
        typoutput -> Oid,
        typreceive -> Oid,
        typsend -> Oid,
        typmodin -> Oid,
        typmodout -> Oid,
        typanalyze -> Oid,
        typalign -> Char,
        typstorage -> Char,
        typnotnull -> Bool,
        typbasetype -> Oid,
        typtypmod -> Integer,
        typndims -> Integer,
        typcollation -> Oid,
        typdefaultbin -> Nullable<Bytea>,
        typdefault -> Nullable<Text>,
    }
}

#[cfg(feature = "postgres_17")]
table! {
    pg_type (oid, typname, typnamespace) {
        oid -> Oid,
        typname -> Text,
        typnamespace -> Oid,
        typowner -> Oid,
        typlen -> SmallInt,
        typbyval -> Bool,
        typtype -> Char,
        typcategory -> Char,
        typispreferred -> Bool,
        typisdefined -> Bool,
        typdelim -> Char,
        typrelid -> Oid,
        typsubscript -> Oid,
        typelem -> Oid,
        typarray -> Oid,
        typinput -> Oid,
        typoutput -> Oid,
        typreceive -> Oid,
        typsend -> Oid,
        typmodin -> Oid,
        typmodout -> Oid,
        typanalyze -> Oid,
        typalign -> Char,
        typstorage -> Char,
        typnotnull -> Bool,
        typbasetype -> Oid,
        typtypmod -> Integer,
        typndims -> Integer,
        typcollation -> Oid,
        typdefaultbin -> Nullable<Bytea>,
        typdefault -> Nullable<Text>,
    }
}

table! {
    pg_operator (oid, oprname, oprnamespace) {
        oid -> Oid,
        oprname -> Text,
        oprnamespace -> Oid,
        oprowner -> Oid,
        oprkind -> Char,
        oprcanmerge -> Bool,
        oprcanhash -> Bool,
        oprleft -> Oid,
        oprright -> Oid,
        oprresult -> Oid,
        oprcom -> Oid,
        oprnegate -> Oid,
        oprcode -> Oid,
        oprrest -> Oid,
        oprjoin -> Oid,
    }
}

table! {
    pg_constraint (oid, conname, connamespace) {
        oid -> Oid,
        conname -> Text,
        connamespace -> Oid,
        contype -> Char,
        condeferrable -> Bool,
        condeferred -> Bool,
        convalidated -> Bool,
        conrelid -> Oid,
        contypid -> Oid,
        conindid -> Oid,
        conparentid -> Oid,
        confrelid -> Oid,
        confupdtype -> Char,
        confdeltype -> Char,
        confmatchtype -> Char,
        conislocal -> Bool,
        coninhcount -> SmallInt,
        connoinherit -> Bool,
        conkey -> Nullable<Array<SmallInt>>,
        confkey -> Nullable<Array<SmallInt>>,
        conpfeqop -> Nullable<Array<Oid>>,
        conppeqop -> Nullable<Array<Oid>>,
        conffeqop -> Nullable<Array<Oid>>,
        confdelsetcols -> Nullable<Array<SmallInt>>,
        conexclop -> Nullable<Array<Oid>>,
    }
}

table! {
    pg_class (oid, relname, relnamespace) {
        oid -> Oid,
        relname -> Text,
        relnamespace -> Oid,
        reltype -> Oid,
        reloftype -> Oid,
        relowner -> Oid,
        relam -> Oid,
        relfilenode -> Oid,
        reltablespace -> Oid,
        relpages -> Integer,
        reltuples -> Float,
        relallvisible -> Integer,
        reltoastrelid -> Oid,
        relhasindex -> Bool,
        relisshared -> Bool,
        relpersistence -> Char,
        relkind -> Char,
        relnatts -> SmallInt,
        relchecks -> SmallInt,
        relhasrules -> Bool,
        relhastriggers -> Bool,
        relhassubclass -> Bool,
        relrowsecurity -> Bool,
        relforcerowsecurity -> Bool,
        relispopulated -> Bool,
        relreplident -> Char,
        relispartition -> Bool,
        relrewrite -> Oid,
    }
}

#[cfg(feature = "postgres_17")]
table! {
    pg_attribute (attrelid, attname, atttypid) {
        attrelid -> Oid,
        attname -> Text,
        atttypid -> Oid,
        attlen -> SmallInt,
        attnum -> SmallInt,
        attcacheoff -> Integer,
        atttypmod -> Integer,
        attndims -> SmallInt,
        attbyval -> Bool,
        attalign -> Char,
        attstorage -> Char,
        attcompression -> Char,
        attnotnull -> Bool,
        atthasdef -> Bool,
        atthasmissing -> Bool,
        attidentity -> Char,
        attgenerated -> Char,
        attisdropped -> Bool,
        attislocal -> Bool,
        attinhcount -> SmallInt,
        attcollation -> Oid,
        attstattarget -> Nullable<SmallInt>,
        attacl -> Nullable<Array<Oid>>,
        attoptions -> Nullable<Array<Text>>,
        attfdwoptions -> Nullable<Array<Text>>,
    }
}

#[cfg(not(feature = "postgres_17"))]
table! {
    pg_attribute (attrelid, attname, atttypid) {
        attrelid -> Oid,
        attname -> Text,
        atttypid -> Oid,
        attlen -> SmallInt,
        attnum -> SmallInt,
        attcacheoff -> Integer,
        atttypmod -> Integer,
        attndims -> SmallInt,
        attbyval -> Bool,
        attalign -> Char,
        attstorage -> Char,
        attnotnull -> Bool,
        atthasdef -> Bool,
        atthasmissing -> Bool,
        attidentity -> Char,
        attgenerated -> Char,
        attisdropped -> Bool,
        attislocal -> Bool,
        attinhcount -> SmallInt,
        attcollation -> Oid,
        attstattarget -> Nullable<SmallInt>,
        attacl -> Nullable<Array<Oid>>,
        attoptions -> Nullable<Array<Text>>,
        attfdwoptions -> Nullable<Array<Text>>,
    }
}

table! {
    pg_trigger (oid) {
        oid -> Oid,
        tgrelid -> Oid,
        tgparentid -> Oid,
        tgname -> Text,
        tgfoid -> Oid,
        tgtype -> SmallInt,
        tgenabled -> Char,
        tgisinternal -> Bool,
        tgconstrrelid -> Oid,
        tgconstrindid -> Oid,
        tgconstraint -> Oid,
        tgdeferrable -> Bool,
        tginitdeferred -> Bool,
        tgnargs -> SmallInt,
        tgattr -> Array<SmallInt>,
        tgargs -> Bytea,
        tgoldtable -> Nullable<Text>,
        tgnewtable -> Nullable<Text>,
    }
}

table! {
    pg_enum (oid) {
        oid -> Oid,
        enumtypid -> Oid,
        enumsortorder -> Float,
        enumlabel -> Text,
    }
}

table! {
    geometry_columns (f_table_catalog, f_table_schema, f_table_name, f_geometry_column) {
        f_table_catalog -> Text,
        f_table_schema -> Text,
        f_table_name -> Text,
        f_geometry_column -> Text,
        coord_dimension -> Integer,
        srid -> Integer,
        r#type -> Text
    }
}

table! {
    geography_columns (f_table_catalog, f_table_schema, f_table_name, f_geography_column) {
        f_table_catalog -> Text,
        f_table_schema -> Text,
        f_table_name -> Text,
        f_geography_column -> Text,
        coord_dimension -> Integer,
        srid -> Integer,
        r#type -> Text
    }
}

allow_tables_to_appear_in_same_query!(
    table_constraints,
    key_column_usage,
    tables,
    constraint_column_usage,
    columns,
    referential_constraints
);
allow_tables_to_appear_in_same_query!(check_constraints, table_constraints);
allow_tables_to_appear_in_same_query!(pg_trigger, pg_class, pg_namespace);
allow_tables_to_appear_in_same_query!(pg_operator, pg_proc, pg_type);
allow_tables_to_appear_in_same_query!(pg_namespace, pg_proc);
allow_tables_to_appear_in_same_query!(pg_constraint, pg_depend, pg_proc);
allow_tables_to_appear_in_same_query!(pg_attribute, pg_type);
allow_tables_to_appear_in_same_query!(pg_depend, pg_extension);
allow_tables_to_appear_in_same_query!(columns, pg_attribute, pg_class, pg_index);
allow_tables_to_appear_in_same_query!(pg_extension, pg_namespace);
allow_tables_to_appear_in_same_query!(check_constraints, constraint_column_usage);
allow_tables_to_appear_in_same_query!(pg_constraint, pg_namespace);
