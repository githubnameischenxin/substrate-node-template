use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};


#[test]
fn create_kitty_works() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
        assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

        System::assert_last_event(Event::KittyCreated { who: account_id, kitty_id: kitty_id, kitty: KittiesModule::kitties(kitty_id).unwrap() }.into() );

        crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
        assert_noop!(
            KittiesModule::create(RuntimeOrigin::signed(account_id)),
            Error::<Test>::InvalidKittyId
        );
    })
}

#[test]
fn breed_kitty_works() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
            Error::<Test>::InvalidKittyId
        );

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
            Error::<Test>::SameKittyId
        );

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

        assert_ok!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1)
        );

        let breed_kitty_id = 2;
        assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
        assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(breed_kitty_id), Some((kitty_id, kitty_id + 1)));

        System::assert_last_event(Event::KittyBreed { who: account_id, kitty_id: breed_kitty_id, kitty: KittiesModule::kitties(breed_kitty_id).unwrap() }.into() );
    })
}

#[test]
fn transfer_kitty_works() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        
        let account_id = 1;
        let to = 2;
        
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

        assert_noop!(KittiesModule::transfer(RuntimeOrigin::signed(to), account_id, kitty_id), Error::<Test>::NotOwner);

        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), to, kitty_id));

        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(to));

        System::assert_last_event(Event::KittyTransferred { who: account_id, to: to, kitty_id: kitty_id }.into() );

        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(to), account_id, kitty_id));

        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

        System::assert_last_event(Event::KittyTransferred { who: to, to: account_id, kitty_id: kitty_id }.into() );
    })
}