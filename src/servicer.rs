use crate::utils::{biguint_to_scalar, generate_public_key, hash_sha256};
use k256::{EncodedPoint, NonZeroScalar, ProjectivePoint};

pub struct Servicer {
    pub id: u8,
    pub R: EncodedPoint,
    pub S: NonZeroScalar,
    pub PK: Option<EncodedPoint>,
    pub PKas: EncodedPoint,
}

impl Servicer {
    pub fn is_valid(&self) -> bool {
        // left
        let left = generate_public_key(&self.S);
        let left = left.decode::<ProjectivePoint>().unwrap();

        // right
        let mut hash = self.R.to_bytes().to_vec();

        hash.push(00 as u8);
        hash.push(self.id as u8);
        let hash = hash_sha256(&hash);
        let hash = biguint_to_scalar(&hash);
        let hash = NonZeroScalar::new(hash).unwrap();

        let PKas = self.PKas.decode::<ProjectivePoint>().unwrap();
        let R = self.R.decode::<ProjectivePoint>().unwrap();
        let right = R + PKas * &*hash;

        println!("left. {:?}", left);
        println!("right. {:?}", right);

        // check
        left == right
    }
}

#[test]
fn test_verify_servicer() {
    use crate::authority::Authority;

    let mut rng = rand::thread_rng();

    let authority = Authority::random(rng);
    let servicer = authority.register_servicer(10, &mut rng);

    let result = servicer.is_valid();
    assert!(result);
}
