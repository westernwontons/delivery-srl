CREATE MIGRATION m1vno2bxxj3tu7h44wwiztlzt3dvmm5coe5uh4e33zehoix3ppozia
    ONTO m1g6av67r57wtsm4kuepy5u3sk5iosmvtys4jsavvhyuimr6gi3yka
{
  ALTER TYPE default::Customer {
      CREATE INDEX ON (.document_id);
  };
};
