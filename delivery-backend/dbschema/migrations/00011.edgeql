CREATE MIGRATION m1xw637ofy5bzqijcmby4u57pco5g4wfphp5hwv46a5lqxkotz2enq
    ONTO m1yob45drimou4mzp4cc62tsl7cd5sbyjiannoj5kfurbbwmhzq6uq
{
  ALTER TYPE default::Appliance {
      ALTER PROPERTY expiry_date {
          RENAME TO expiration_date;
      };
  };
};
