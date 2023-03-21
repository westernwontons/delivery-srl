CREATE MIGRATION m1wgzk43aph7g4sk37igny5pptebxbair45q5ajbilowyfmhhdpm6a
    ONTO initial
{
  CREATE FUTURE nonrecursive_access_policies;
  CREATE TYPE default::Address {
      CREATE REQUIRED PROPERTY additional -> std::str;
      CREATE REQUIRED PROPERTY county -> std::str;
      CREATE REQUIRED PROPERTY number -> std::str;
      CREATE REQUIRED PROPERTY street -> std::str;
  };
  CREATE SCALAR TYPE default::OperationPerformed EXTENDING enum<VTP, INT, PIF, RGAZ, VGAZ>;
  CREATE TYPE default::Appliance {
      CREATE REQUIRED PROPERTY date -> std::datetime;
      CREATE REQUIRED PROPERTY expiry_date -> std::datetime;
      CREATE REQUIRED PROPERTY is_active -> std::bool {
          SET default := true;
      };
      CREATE REQUIRED PROPERTY is_expired -> std::bool {
          SET default := false;
      };
      CREATE REQUIRED PROPERTY is_handled -> std::bool {
          SET default := false;
      };
      CREATE REQUIRED PROPERTY last_updated -> std::datetime {
          SET default := (std::datetime_current());
      };
      CREATE REQUIRED PROPERTY manufacturer -> std::str;
      CREATE REQUIRED PROPERTY model -> std::str;
      CREATE REQUIRED PROPERTY number -> std::str;
      CREATE REQUIRED PROPERTY observations -> std::str;
      CREATE REQUIRED PROPERTY operation_performed -> default::OperationPerformed;
      CREATE REQUIRED PROPERTY type -> std::str;
      CREATE REQUIRED PROPERTY warranty -> std::datetime;
      CREATE REQUIRED PROPERTY year_of_manufacture -> std::str;
  };
  CREATE TYPE default::Client {
      CREATE REQUIRED LINK address -> default::Address;
      CREATE REQUIRED LINK appliance -> default::Appliance;
      CREATE REQUIRED PROPERTY document_id -> std::str;
      CREATE REQUIRED PROPERTY name -> std::str;
  };
};
