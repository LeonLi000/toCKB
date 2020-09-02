// Generated by Molecule 0.6.1

use molecule :: prelude :: * ;
use super :: basic :: * ;
# [ derive ( Clone ) ] pub struct ToCKBCellData ( molecule :: bytes :: Bytes ) ; impl :: core :: fmt :: LowerHex for ToCKBCellData { fn fmt ( & self , f : & mut :: core :: fmt :: Formatter ) -> :: core :: fmt :: Result { use molecule :: hex_string ; if f . alternate ( ) { write ! ( f , "0x" ) ? ; } write ! ( f , "{}" , hex_string ( self . as_slice ( ) ) ) } } impl :: core :: fmt :: Debug for ToCKBCellData { fn fmt ( & self , f : & mut :: core :: fmt :: Formatter ) -> :: core :: fmt :: Result { write ! ( f , "{}({:#x})" , Self :: NAME , self ) } } impl :: core :: fmt :: Display for ToCKBCellData { fn fmt ( & self , f : & mut :: core :: fmt :: Formatter ) -> :: core :: fmt :: Result { write ! ( f , "{} {{ " , Self :: NAME ) ? ; write ! ( f , "{}: {}" , "status" , self . status ( ) ) ? ; write ! ( f , ", {}: {}" , "lot_size" , self . lot_size ( ) ) ? ; write ! ( f , ", {}: {}" , "user_lockscript" , self . user_lockscript ( ) ) ? ; write ! ( f , ", {}: {}" , "x_lock_address" , self . x_lock_address ( ) ) ? ; write ! ( f , ", {}: {}" , "signer_lockscript" , self . signer_lockscript ( ) ) ? ; write ! ( f , ", {}: {}" , "x_unlock_address" , self . x_unlock_address ( ) ) ? ; write ! ( f , ", {}: {}" , "redeemer_lockscript" , self . redeemer_lockscript ( ) ) ? ; write ! ( f , ", {}: {}" , "liquidation_trigger_lockscript" , self . liquidation_trigger_lockscript ( ) ) ? ; let extra_count = self . count_extra_fields ( ) ; if extra_count != 0 { write ! ( f , ", .. ({} fields)" , extra_count ) ? ; } write ! ( f , " }}" ) } } impl :: core :: default :: Default for ToCKBCellData { fn default ( ) -> Self { let v : Vec < u8 > = vec ! [ 2 , 1 , 0 , 0 , 36 , 0 , 0 , 0 , 37 , 0 , 0 , 0 , 38 , 0 , 0 , 0 , 91 , 0 , 0 , 0 , 95 , 0 , 0 , 0 , 148 , 0 , 0 , 0 , 152 , 0 , 0 , 0 , 205 , 0 , 0 , 0 , 0 , 0 , 53 , 0 , 0 , 0 , 16 , 0 , 0 , 0 , 48 , 0 , 0 , 0 , 49 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 53 , 0 , 0 , 0 , 16 , 0 , 0 , 0 , 48 , 0 , 0 , 0 , 49 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 53 , 0 , 0 , 0 , 16 , 0 , 0 , 0 , 48 , 0 , 0 , 0 , 49 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 53 , 0 , 0 , 0 , 16 , 0 , 0 , 0 , 48 , 0 , 0 , 0 , 49 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , ] ; ToCKBCellData :: new_unchecked ( v . into ( ) ) } } impl ToCKBCellData { pub const FIELD_COUNT : usize = 8 ; pub fn total_size ( & self ) -> usize { molecule :: unpack_number ( self . as_slice ( ) ) as usize } pub fn field_count ( & self ) -> usize { if self . total_size ( ) == molecule :: NUMBER_SIZE { 0 } else { ( molecule :: unpack_number ( & self . as_slice ( ) [ molecule :: NUMBER_SIZE .. ] ) as usize / 4 ) - 1 } } pub fn count_extra_fields ( & self ) -> usize { self . field_count ( ) - Self :: FIELD_COUNT } pub fn has_extra_fields ( & self ) -> bool { Self :: FIELD_COUNT != self . field_count ( ) } pub fn status ( & self ) -> Byte { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 4 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 8 .. ] ) as usize ; Byte :: new_unchecked ( self . 0 . slice ( start .. end ) ) } pub fn lot_size ( & self ) -> Byte { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 8 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 12 .. ] ) as usize ; Byte :: new_unchecked ( self . 0 . slice ( start .. end ) ) } pub fn user_lockscript ( & self ) -> Script { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 12 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 16 .. ] ) as usize ; Script :: new_unchecked ( self . 0 . slice ( start .. end ) ) } pub fn x_lock_address ( & self ) -> Bytes { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 16 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 20 .. ] ) as usize ; Bytes :: new_unchecked ( self . 0 . slice ( start .. end ) ) } pub fn signer_lockscript ( & self ) -> Script { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 20 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 24 .. ] ) as usize ; Script :: new_unchecked ( self . 0 . slice ( start .. end ) ) } pub fn x_unlock_address ( & self ) -> Bytes { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 24 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 28 .. ] ) as usize ; Bytes :: new_unchecked ( self . 0 . slice ( start .. end ) ) } pub fn redeemer_lockscript ( & self ) -> Script { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 28 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 32 .. ] ) as usize ; Script :: new_unchecked ( self . 0 . slice ( start .. end ) ) } pub fn liquidation_trigger_lockscript ( & self ) -> Script { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 32 .. ] ) as usize ; if self . has_extra_fields ( ) { let end = molecule :: unpack_number ( & slice [ 36 .. ] ) as usize ; Script :: new_unchecked ( self . 0 . slice ( start .. end ) ) } else { Script :: new_unchecked ( self . 0 . slice ( start .. ) ) } } pub fn as_reader < 'r > ( & 'r self ) -> ToCKBCellDataReader < 'r > { ToCKBCellDataReader :: new_unchecked ( self . as_slice ( ) ) } } impl molecule :: prelude :: Entity for ToCKBCellData { type Builder = ToCKBCellDataBuilder ; const NAME : & 'static str = "ToCKBCellData" ; fn new_unchecked ( data : molecule :: bytes :: Bytes ) -> Self { ToCKBCellData ( data ) } fn as_bytes ( & self ) -> molecule :: bytes :: Bytes { self . 0 . clone ( ) } fn as_slice ( & self ) -> & [ u8 ] { & self . 0 [ .. ] } fn from_slice ( slice : & [ u8 ] ) -> molecule :: error :: VerificationResult < Self > { ToCKBCellDataReader :: from_slice ( slice ) . map ( | reader | reader . to_entity ( ) ) } fn from_compatible_slice ( slice : & [ u8 ] ) -> molecule :: error :: VerificationResult < Self > { ToCKBCellDataReader :: from_compatible_slice ( slice ) . map ( | reader | reader . to_entity ( ) ) } fn new_builder ( ) -> Self :: Builder { :: core :: default :: Default :: default ( ) } fn as_builder ( self ) -> Self :: Builder { Self :: new_builder ( ) . status ( self . status ( ) ) . lot_size ( self . lot_size ( ) ) . user_lockscript ( self . user_lockscript ( ) ) . x_lock_address ( self . x_lock_address ( ) ) . signer_lockscript ( self . signer_lockscript ( ) ) . x_unlock_address ( self . x_unlock_address ( ) ) . redeemer_lockscript ( self . redeemer_lockscript ( ) ) . liquidation_trigger_lockscript ( self . liquidation_trigger_lockscript ( ) ) } }
# [ derive ( Clone , Copy ) ] pub struct ToCKBCellDataReader < 'r > ( & 'r [ u8 ] ) ; impl < 'r > :: core :: fmt :: LowerHex for ToCKBCellDataReader < 'r > { fn fmt ( & self , f : & mut :: core :: fmt :: Formatter ) -> :: core :: fmt :: Result { use molecule :: hex_string ; if f . alternate ( ) { write ! ( f , "0x" ) ? ; } write ! ( f , "{}" , hex_string ( self . as_slice ( ) ) ) } } impl < 'r > :: core :: fmt :: Debug for ToCKBCellDataReader < 'r > { fn fmt ( & self , f : & mut :: core :: fmt :: Formatter ) -> :: core :: fmt :: Result { write ! ( f , "{}({:#x})" , Self :: NAME , self ) } } impl < 'r > :: core :: fmt :: Display for ToCKBCellDataReader < 'r > { fn fmt ( & self , f : & mut :: core :: fmt :: Formatter ) -> :: core :: fmt :: Result { write ! ( f , "{} {{ " , Self :: NAME ) ? ; write ! ( f , "{}: {}" , "status" , self . status ( ) ) ? ; write ! ( f , ", {}: {}" , "lot_size" , self . lot_size ( ) ) ? ; write ! ( f , ", {}: {}" , "user_lockscript" , self . user_lockscript ( ) ) ? ; write ! ( f , ", {}: {}" , "x_lock_address" , self . x_lock_address ( ) ) ? ; write ! ( f , ", {}: {}" , "signer_lockscript" , self . signer_lockscript ( ) ) ? ; write ! ( f , ", {}: {}" , "x_unlock_address" , self . x_unlock_address ( ) ) ? ; write ! ( f , ", {}: {}" , "redeemer_lockscript" , self . redeemer_lockscript ( ) ) ? ; write ! ( f , ", {}: {}" , "liquidation_trigger_lockscript" , self . liquidation_trigger_lockscript ( ) ) ? ; let extra_count = self . count_extra_fields ( ) ; if extra_count != 0 { write ! ( f , ", .. ({} fields)" , extra_count ) ? ; } write ! ( f , " }}" ) } } impl < 'r > ToCKBCellDataReader < 'r > { pub const FIELD_COUNT : usize = 8 ; pub fn total_size ( & self ) -> usize { molecule :: unpack_number ( self . as_slice ( ) ) as usize } pub fn field_count ( & self ) -> usize { if self . total_size ( ) == molecule :: NUMBER_SIZE { 0 } else { ( molecule :: unpack_number ( & self . as_slice ( ) [ molecule :: NUMBER_SIZE .. ] ) as usize / 4 ) - 1 } } pub fn count_extra_fields ( & self ) -> usize { self . field_count ( ) - Self :: FIELD_COUNT } pub fn has_extra_fields ( & self ) -> bool { Self :: FIELD_COUNT != self . field_count ( ) } pub fn status ( & self ) -> ByteReader < 'r > { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 4 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 8 .. ] ) as usize ; ByteReader :: new_unchecked ( & self . as_slice ( ) [ start .. end ] ) } pub fn lot_size ( & self ) -> ByteReader < 'r > { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 8 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 12 .. ] ) as usize ; ByteReader :: new_unchecked ( & self . as_slice ( ) [ start .. end ] ) } pub fn user_lockscript ( & self ) -> ScriptReader < 'r > { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 12 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 16 .. ] ) as usize ; ScriptReader :: new_unchecked ( & self . as_slice ( ) [ start .. end ] ) } pub fn x_lock_address ( & self ) -> BytesReader < 'r > { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 16 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 20 .. ] ) as usize ; BytesReader :: new_unchecked ( & self . as_slice ( ) [ start .. end ] ) } pub fn signer_lockscript ( & self ) -> ScriptReader < 'r > { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 20 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 24 .. ] ) as usize ; ScriptReader :: new_unchecked ( & self . as_slice ( ) [ start .. end ] ) } pub fn x_unlock_address ( & self ) -> BytesReader < 'r > { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 24 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 28 .. ] ) as usize ; BytesReader :: new_unchecked ( & self . as_slice ( ) [ start .. end ] ) } pub fn redeemer_lockscript ( & self ) -> ScriptReader < 'r > { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 28 .. ] ) as usize ; let end = molecule :: unpack_number ( & slice [ 32 .. ] ) as usize ; ScriptReader :: new_unchecked ( & self . as_slice ( ) [ start .. end ] ) } pub fn liquidation_trigger_lockscript ( & self ) -> ScriptReader < 'r > { let slice = self . as_slice ( ) ; let start = molecule :: unpack_number ( & slice [ 32 .. ] ) as usize ; if self . has_extra_fields ( ) { let end = molecule :: unpack_number ( & slice [ 36 .. ] ) as usize ; ScriptReader :: new_unchecked ( & self . as_slice ( ) [ start .. end ] ) } else { ScriptReader :: new_unchecked ( & self . as_slice ( ) [ start .. ] ) } } } impl < 'r > molecule :: prelude :: Reader < 'r > for ToCKBCellDataReader < 'r > { type Entity = ToCKBCellData ; const NAME : & 'static str = "ToCKBCellDataReader" ; fn to_entity ( & self ) -> Self :: Entity { Self :: Entity :: new_unchecked ( self . as_slice ( ) . to_owned ( ) . into ( ) ) } fn new_unchecked ( slice : & 'r [ u8 ] ) -> Self { ToCKBCellDataReader ( slice ) } fn as_slice ( & self ) -> & 'r [ u8 ] { self . 0 } fn verify ( slice : & [ u8 ] , compatible : bool ) -> molecule :: error :: VerificationResult < ( ) > { use molecule :: verification_error as ve ; let slice_len = slice . len ( ) ; if slice_len < molecule :: NUMBER_SIZE { return ve ! ( Self , HeaderIsBroken , molecule :: NUMBER_SIZE , slice_len ) ; } let total_size = molecule :: unpack_number ( slice ) as usize ; if slice_len != total_size { return ve ! ( Self , TotalSizeNotMatch , total_size , slice_len ) ; } if slice_len == molecule :: NUMBER_SIZE && Self :: FIELD_COUNT == 0 { return Ok ( ( ) ) ; } if slice_len < molecule :: NUMBER_SIZE * 2 { return ve ! ( Self , HeaderIsBroken , molecule :: NUMBER_SIZE * 2 , slice_len ) ; } let offset_first = molecule :: unpack_number ( & slice [ molecule :: NUMBER_SIZE .. ] ) as usize ; if offset_first % 4 != 0 || offset_first < molecule :: NUMBER_SIZE * 2 { return ve ! ( Self , OffsetsNotMatch ) ; } let field_count = offset_first / 4 - 1 ; if field_count < Self :: FIELD_COUNT { return ve ! ( Self , FieldCountNotMatch , Self :: FIELD_COUNT , field_count ) ; } else if ! compatible && field_count > Self :: FIELD_COUNT { return ve ! ( Self , FieldCountNotMatch , Self :: FIELD_COUNT , field_count ) ; } ; let header_size = molecule :: NUMBER_SIZE * ( field_count + 1 ) ; if slice_len < header_size { return ve ! ( Self , HeaderIsBroken , header_size , slice_len ) ; } let mut offsets : Vec < usize > = slice [ molecule :: NUMBER_SIZE .. ] . chunks ( molecule :: NUMBER_SIZE ) . take ( field_count ) . map ( | x | molecule :: unpack_number ( x ) as usize ) . collect ( ) ; offsets . push ( total_size ) ; if offsets . windows ( 2 ) . any ( | i | i [ 0 ] > i [ 1 ] ) { return ve ! ( Self , OffsetsNotMatch ) ; } ByteReader :: verify ( & slice [ offsets [ 0 ] .. offsets [ 1 ] ] , compatible ) ? ; ByteReader :: verify ( & slice [ offsets [ 1 ] .. offsets [ 2 ] ] , compatible ) ? ; ScriptReader :: verify ( & slice [ offsets [ 2 ] .. offsets [ 3 ] ] , compatible ) ? ; BytesReader :: verify ( & slice [ offsets [ 3 ] .. offsets [ 4 ] ] , compatible ) ? ; ScriptReader :: verify ( & slice [ offsets [ 4 ] .. offsets [ 5 ] ] , compatible ) ? ; BytesReader :: verify ( & slice [ offsets [ 5 ] .. offsets [ 6 ] ] , compatible ) ? ; ScriptReader :: verify ( & slice [ offsets [ 6 ] .. offsets [ 7 ] ] , compatible ) ? ; ScriptReader :: verify ( & slice [ offsets [ 7 ] .. offsets [ 8 ] ] , compatible ) ? ; Ok ( ( ) ) } }
# [ derive ( Debug , Default ) ] pub struct ToCKBCellDataBuilder { pub ( crate ) status : Byte , pub ( crate ) lot_size : Byte , pub ( crate ) user_lockscript : Script , pub ( crate ) x_lock_address : Bytes , pub ( crate ) signer_lockscript : Script , pub ( crate ) x_unlock_address : Bytes , pub ( crate ) redeemer_lockscript : Script , pub ( crate ) liquidation_trigger_lockscript : Script , } impl ToCKBCellDataBuilder { pub const FIELD_COUNT : usize = 8 ; pub fn status ( mut self , v : Byte ) -> Self { self . status = v ; self } pub fn lot_size ( mut self , v : Byte ) -> Self { self . lot_size = v ; self } pub fn user_lockscript ( mut self , v : Script ) -> Self { self . user_lockscript = v ; self } pub fn x_lock_address ( mut self , v : Bytes ) -> Self { self . x_lock_address = v ; self } pub fn signer_lockscript ( mut self , v : Script ) -> Self { self . signer_lockscript = v ; self } pub fn x_unlock_address ( mut self , v : Bytes ) -> Self { self . x_unlock_address = v ; self } pub fn redeemer_lockscript ( mut self , v : Script ) -> Self { self . redeemer_lockscript = v ; self } pub fn liquidation_trigger_lockscript ( mut self , v : Script ) -> Self { self . liquidation_trigger_lockscript = v ; self } } impl molecule :: prelude :: Builder for ToCKBCellDataBuilder { type Entity = ToCKBCellData ; const NAME : & 'static str = "ToCKBCellDataBuilder" ; fn expected_length ( & self ) -> usize { molecule :: NUMBER_SIZE * ( Self :: FIELD_COUNT + 1 ) + self . status . as_slice ( ) . len ( ) + self . lot_size . as_slice ( ) . len ( ) + self . user_lockscript . as_slice ( ) . len ( ) + self . x_lock_address . as_slice ( ) . len ( ) + self . signer_lockscript . as_slice ( ) . len ( ) + self . x_unlock_address . as_slice ( ) . len ( ) + self . redeemer_lockscript . as_slice ( ) . len ( ) + self . liquidation_trigger_lockscript . as_slice ( ) . len ( ) } fn write < W : :: molecule :: io :: Write > ( & self , writer : & mut W ) -> :: molecule :: io :: Result < ( ) > { let mut total_size = molecule :: NUMBER_SIZE * ( Self :: FIELD_COUNT + 1 ) ; let mut offsets = Vec :: with_capacity ( Self :: FIELD_COUNT ) ; offsets . push ( total_size ) ; total_size += self . status . as_slice ( ) . len ( ) ; offsets . push ( total_size ) ; total_size += self . lot_size . as_slice ( ) . len ( ) ; offsets . push ( total_size ) ; total_size += self . user_lockscript . as_slice ( ) . len ( ) ; offsets . push ( total_size ) ; total_size += self . x_lock_address . as_slice ( ) . len ( ) ; offsets . push ( total_size ) ; total_size += self . signer_lockscript . as_slice ( ) . len ( ) ; offsets . push ( total_size ) ; total_size += self . x_unlock_address . as_slice ( ) . len ( ) ; offsets . push ( total_size ) ; total_size += self . redeemer_lockscript . as_slice ( ) . len ( ) ; offsets . push ( total_size ) ; total_size += self . liquidation_trigger_lockscript . as_slice ( ) . len ( ) ; writer . write_all ( & molecule :: pack_number ( total_size as molecule :: Number ) ) ? ; for offset in offsets . into_iter ( ) { writer . write_all ( & molecule :: pack_number ( offset as molecule :: Number ) ) ? ; } writer . write_all ( self . status . as_slice ( ) ) ? ; writer . write_all ( self . lot_size . as_slice ( ) ) ? ; writer . write_all ( self . user_lockscript . as_slice ( ) ) ? ; writer . write_all ( self . x_lock_address . as_slice ( ) ) ? ; writer . write_all ( self . signer_lockscript . as_slice ( ) ) ? ; writer . write_all ( self . x_unlock_address . as_slice ( ) ) ? ; writer . write_all ( self . redeemer_lockscript . as_slice ( ) ) ? ; writer . write_all ( self . liquidation_trigger_lockscript . as_slice ( ) ) ? ; Ok ( ( ) ) } fn build ( & self ) -> Self :: Entity { let mut inner = Vec :: with_capacity ( self . expected_length ( ) ) ; self . write ( & mut inner ) . unwrap_or_else ( | _ | panic ! ( "{} build should be ok" , Self :: NAME ) ) ; ToCKBCellData :: new_unchecked ( inner . into ( ) ) } }
