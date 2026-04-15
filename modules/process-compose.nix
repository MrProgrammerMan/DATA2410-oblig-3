{ inputs, ... }: {
  imports = [
    inputs.process-compose-flake.flakeModule
  ];
  perSystem = { ... }: {
    process-compose.default = {
      imports = [
        inputs.services-flake.processComposeModules.default
      ];
      services = {
        postgres."pg1" = 
          let
            dir = ../migrations;
          in {
            enable = true;
            initialScript = {
              before = ''
                CREATE USER postgres WITH password 'postgres';
              '';
              after = ''
                GRANT ALL PRIVILEGES ON DATABASE db TO postgres;
                \c db
                GRANT ALL ON SCHEMA public TO postgres;
                GRANT ALL ON ALL TABLES IN SCHEMA public TO postgres;
                GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO postgres;
              '';
            };
            initialDatabases = [
              {
                name = "db";
                schemas = map (name: dir + "/${name}") (builtins.attrNames (builtins.readDir dir)); # This WILL cause problems if files cannot be sorted lexigraphically, i.e. 1.sql, 2.sql, 10.sql will result in wrong order!
              }
            ];
          };
      };
    };
  };
}