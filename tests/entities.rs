use inv_tdd::core::entities::asset::Asset;


#[test]
fn can_make_asset() {
    let asset_name = "New Asset".to_string();
    let asset_sku = "new_asset".to_string();
    let asset = Asset::new(asset_name.clone(), asset_sku.clone());
    assert_eq!(asset.name, asset_name);
    assert_eq!(asset.sku, asset_sku);
    dbg!(asset);
    }
