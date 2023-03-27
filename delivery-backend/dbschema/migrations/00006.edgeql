CREATE MIGRATION m1wa6dt45yaelq25ssehm4ichplj44lhcm4foihv676wucbiolz6ca
    ONTO m1vno2bxxj3tu7h44wwiztlzt3dvmm5coe5uh4e33zehoix3ppozia
{
  ALTER TYPE default::Appliance {
      DROP PROPERTY is_active;
  };
  ALTER TYPE default::Appliance {
      ALTER PROPERTY is_expired {
          RENAME TO expired;
      };
  };
  CREATE SCALAR TYPE default::CustomerStatus EXTENDING enum<ACTIVE, INACTIVE>;
  ALTER TYPE default::Appliance {
      CREATE REQUIRED PROPERTY status -> default::CustomerStatus {
          SET default := (default::CustomerStatus.ACTIVE);
      };
  };
};
