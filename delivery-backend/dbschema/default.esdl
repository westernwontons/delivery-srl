module default {
	scalar type OperationPerformed extending enum<VTP, INT, PIF, RGAZ, VGAZ>; # operatia_efect
	
	type Address {
		required property county -> str; # judet
		required property street -> str; # strada
		required property number -> str; # nr
		required property additional -> str; # bl_sc_ap
	}

	type Appliance {
		required property manufacturer -> str; # producator
		required property year_of_manufacture -> str; # nr_fabr_an_fabr
		required property model -> str; # model
		required property type -> str; # tip
		required property warranty -> datetime; # garantia
		required property operation_performed -> OperationPerformed; # operatia_efect
		required property number -> str; # numar
		required property date -> datetime; # data
		required property expiry_date -> datetime; # scadenta
		property observations -> str; # observatii
		required property last_updated -> datetime {
			default := datetime_current();
		};
		required property is_active -> bool {
			default := true;
		}
		required property is_expired -> bool {
			default := false;
		}
		required property is_handled -> bool {
			default := false;
		}
	}

	type Client {
		required property document_id -> str; # dosar
		required property name -> str; # destinator_utilizator
		required link address -> Address; # adresa
		required link appliance -> Appliance;
	}
}
