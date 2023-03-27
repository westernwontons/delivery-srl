CREATE MIGRATION m137xmiqcl7nfdk7pk7bmbxzh6lnxpin3abiww7f2eexw3knnzmphq
    ONTO m15u25wnoxm3kbgavaj3wi3daqfsqcblvqdcncopg4zoqutmjsdveq
{
  ALTER TYPE default::Appliance {
      DROP PROPERTY status;
  };
  ALTER TYPE default::Customer {
      CREATE REQUIRED PROPERTY status -> default::CustomerStatus {
          SET default := (default::CustomerStatus.ACTIVE);
      };
  };
};
