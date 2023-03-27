CREATE MIGRATION m1zi2vbqghnqwotanhbc527irctzehobjjyhhanczw544tbx6myofa
    ONTO m137xmiqcl7nfdk7pk7bmbxzh6lnxpin3abiww7f2eexw3knnzmphq
{
  ALTER TYPE default::Appliance {
      ALTER PROPERTY last_updated {
          RESET readonly;
      };
  };
};
