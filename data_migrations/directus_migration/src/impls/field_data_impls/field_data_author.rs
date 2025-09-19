//! Submodule providing helpers methods to work with the `FieldDatum` author.

use core_structures::{User, tables::insertables::UserSettable};
use diesel::PgConnection;
use web_common_traits::database::{BackendInsertableVariant, Insertable};

use crate::structs::FieldDatumWrapper;

impl FieldDatumWrapper {
    /// Returns the author of the field datum if it exists.
    pub fn author(&self, portal: &mut PgConnection) -> anyhow::Result<User> {
        if let Some(collector_fullname) = &self.as_ref().collector_fullname {
            return dispatch_user_from_name(collector_fullname.as_str(), portal);
        }
        if let Some(user) = self.dispatch_user_from_picture_panel(portal)? {
            return Ok(user);
        }
        todo!("dispatch author retrieval to field_data_author module")
    }

    fn dispatch_user_from_picture_panel(
        &self,
        portal: &mut PgConnection,
    ) -> anyhow::Result<Option<User>> {
        let Some(picture_panel) = &self.as_ref().picture_panel else {
            return Ok(None);
        };

        let Some(picture_general) = &self.as_ref().picture_general else {
            return Ok(None);
        };

        let Some(qfield_project) = &self.as_ref().qfield_project else {
            return Ok(None);
        };

        if picture_panel.starts_with("DCIM/Audrey_layer/")
            || picture_panel.starts_with("DCIM/Audrey_le_cabec/")
        {
            return Ok(Some(get_or_insert_user("Audrey", "Le Cabec", portal)?));
        }

        if picture_panel.starts_with("DCIM/edouard_brulhart_mw_2023/")
            || picture_panel.starts_with("DCIM/SBL_20004_2023/")
        {
            return Ok(Some(get_or_insert_user("Edouard", "Brülhart", portal)?));
        }

        if picture_panel.starts_with("DCIM/Teo_Valentino/")
            || picture_panel.starts_with("DCIM/teo_valentino/")
            || picture_general.starts_with("DCIM/Teo_Valentino/")
        {
            return Ok(Some(get_or_insert_user("Teo", "Valentino", portal)?));
        }

        if picture_panel.starts_with("DCIM/heloise_coen/")
            || picture_panel.starts_with("DCIM/below_ground/")
        {
            return Ok(Some(get_or_insert_user("Héloïse", "Coen", portal)?));
        }

        if picture_panel.starts_with("DCIM/Succulent_greenhouse/") {
            return Ok(Some(get_or_insert_user("Lëndita", "Schwegler", portal)?));
        }

        if picture_panel.starts_with("DCIM/JPEG_") {
            return Ok(Some(get_or_insert_user("Stéphanie", "Guetchueng", portal)?));
        }

        if picture_panel.starts_with("files/") && qfield_project.contains("jbuf") {
            return Ok(Some(get_or_insert_user("Edouard", "Brülhart", portal)?));
        }

        if picture_panel.starts_with("files/") && qfield_project.contains("jbn") {
            return Ok(Some(get_or_insert_user("Emmanuel", "Defossez", portal)?));
        }

        Ok(None)
    }
}

fn dispatch_user_from_name(name: &str, portal: &mut PgConnection) -> anyhow::Result<User> {
    match name {
        "Lysandre Journiac" => get_or_insert_user("Lysandre", "Journiac", portal),
        "Loic Chalmandrier" => get_or_insert_user("Loic", "Chalmandrier", portal),
        "Alžběta Kadlecová" | "Alzbeta" => get_or_insert_user("Alžběta", "Kadlecová", portal),
        "Federico Brigante" => get_or_insert_user("Federico", "Brigante", portal),
        "Emilie Lab" | "Émilie Lab" | "Lab Emilie" => get_or_insert_user("Émilie", "Lab", portal),
        "Mazzarine Laboureau" | "Mazzarine laboureau" => {
            get_or_insert_user("Mazzarine", "Laboureau", portal)
        }
        "Ana Claudia Sima" => get_or_insert_user("Ana Claudia", "Sima", portal),
        "Maëlle Wannier" => get_or_insert_user("Maëlle", "Wannier", portal),
        "Lise Lebrun" => get_or_insert_user("Lise", "Lebrun", portal),
        "Héloïse Coen" => get_or_insert_user("Héloïse", "Coen", portal),
        "Donat Agosti" => get_or_insert_user("Donat", "Agosti", portal),
        "Marco Andreas Stanley Visani" => get_or_insert_user("Marco", "Visani", portal),
        "Disha Tandon" => get_or_insert_user("Disha", "Tandon", portal),
        "Simon Rérat" => get_or_insert_user("Simon", "Rérat", portal),
        "Jade Dandois" => get_or_insert_user("Jade", "Dandois", portal),
        "James Smith" => get_or_insert_user("James", "Smith", portal),
        "Chloé Blanc" => get_or_insert_user("Chloé", "Blanc", portal),
        "Alexandra Slotte" => get_or_insert_user("Alexandra", "Slotte", portal),
        "Edouard Brülhart" => get_or_insert_user("Edouard", "Brülhart", portal),
        "Pierre-Marie Allard" => get_or_insert_user("Pierre-Marie", "Allard", portal),
        "Emmanuel Defossez" => get_or_insert_user("Emmanuel", "Defossez", portal),
        _ => todo!("implement user dispatch from name: {name}"),
    }
}

fn get_or_insert_user(
    first_name: &str,
    last_name: &str,
    portal: &mut PgConnection,
) -> anyhow::Result<User> {
    let users = User::from_first_name_and_last_name(first_name, last_name, portal)?;

    if !users.is_empty() {
        assert_eq!(
            users.len(),
            1,
            "Expected exactly one user with name {first_name} {last_name}, found {}",
            users.len()
        );
        return Ok(users.into_iter().next().unwrap());
    }

    let user: User =
        User::new().last_name(last_name)?.first_name(first_name)?.backend_insert(portal)?;

    Ok(user)
}
