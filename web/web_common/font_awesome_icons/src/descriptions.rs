//! Submodule providing the descriptions for Font Awesome icons.
use super::FAIcon;

impl FAIcon {
    #[must_use]
    #[allow(clippy::too_many_lines)]
    /// Returns the description for the icon.
    pub fn description(&self) -> &str {
        match self {
            FAIcon::Zero => "A numeral 0, representing the number.",
            FAIcon::One => "A numeral 1, representing the number.",
            FAIcon::Two => "A numeral 2, representing the number.",
            FAIcon::Three => "A numeral 3, representing the number.",
            FAIcon::Four => "A numeral 4, representing the number.",
            FAIcon::Five => "A numeral 5, representing the number.",
            FAIcon::Six => "A numeral 6, representing the number.",
            FAIcon::Seven => "A numeral 7, representing the number.",
            FAIcon::Eight => "A numeral 8, representing the number.",
            FAIcon::Nine => "A numeral 9, representing the number.",
            FAIcon::A => "A lowercase letter 'a', representing the letter.",
            FAIcon::AccessibleIcon => {
                "An icon of a person in a wheelchair, indicating accessibility."
            }
            FAIcon::AddressBook => "A book with a person's silhouette, representing a contact list.",
            FAIcon::AddressCard => {
                "A card with a person's silhouette, indicating contact information."
            }
            FAIcon::Algolia => "The logo of Algolia, representing the search engine.",
            FAIcon::AlignCenter => "Text aligned to the center, representing text formatting.",
            FAIcon::AlignJustify => "Text justified, representing text formatting.",
            FAIcon::AlignLeft => "Text aligned to the left, representing text formatting.",
            FAIcon::AlignRight => "Text aligned to the right, representing text formatting.",
            FAIcon::Alipay => "The logo of Alipay, representing the payment platform.",
            FAIcon::AmazonPay => "The logo of Amazon Pay, representing the payment service.",
            FAIcon::Anchor => "An anchor, representing stability or maritime themes.",
            FAIcon::AnchorCircleCheck => {
                "An anchor with a circled check mark, representing stability."
            }
            FAIcon::AnchorCircleExclamation => {
                "An anchor with a circled exclamation mark, representing caution."
            }
            FAIcon::AnchorCircleXmark => "An anchor with a circled `X`, representing instability.",
            FAIcon::AnchorLock => "An anchor with a lock, representing stability and security.",
            FAIcon::Android => "The logo of Android, an operating system for mobile devices.",
            FAIcon::AngleDown => "A downward angle, representing direction.",
            FAIcon::AngleLeft => "A left angle, representing direction.",
            FAIcon::AngleRight => "A right angle, representing direction.",
            FAIcon::AngleUp => "An upward angle, representing direction.",
            FAIcon::AnglesDown => "Two downward angles, representing direction.",
            FAIcon::AnglesLeft => "Two left angles, representing direction.",
            FAIcon::AnglesRight => "Two right angles, representing direction.",
            FAIcon::AnglesUp => "Two upward angles, representing direction.",
            FAIcon::Ankh => "The ankh symbol, representing life in ancient Egypt.",
            FAIcon::Apple => "The logo of Apple, representing the tech company.",
            FAIcon::ApplePay => "The logo of Apple Pay, representing the payment service.",
            FAIcon::AppleWhole => "A whole apple, representing the fruit.",
            FAIcon::Archway => "An architectural arch, representing structure or gateways.",
            FAIcon::ArrowDown => "An arrow pointing down, representing downward direction.",
            FAIcon::ArrowDown19 => {
                "An arrow pointing down with numbers 1 to 9, representing sorting."
            }
            FAIcon::ArrowDown91 => {
                "An arrow pointing down with numbers 9 to 1, representing sorting in reverse."
            }
            FAIcon::ArrowDownAZ => "An arrow pointing down from A to Z, representing sorting.",
            FAIcon::ArrowDownLong => "A long arrow pointing down, indicating downward direction.",
            FAIcon::ArrowDownShortWide => {
                "A short wide arrow pointing down, indicating sorting from smallest to largest."
            }
            FAIcon::ArrowDownUpAcrossLine => {
                "Arrows pointing down and up across a line, representing bidirectional movement."
            }
            FAIcon::ArrowDownUpLock => {
                "An arrow pointing down and up with a lock, representing secure bidirectional movement."
            }
            FAIcon::ArrowDownWideShort => {
                "A short wide arrow pointing down, indicating sorting from largest to smallest."
            }
            FAIcon::ArrowDownZA => {
                "An arrow pointing down with letters Z to A, representing reverse alphabetical order."
            }
            FAIcon::ArrowLeft => "An arrow pointing to the left, indicating direction or back.",
            FAIcon::ArrowLeftLong => {
                "A long arrow pointing left, representing extended backward direction."
            }
            FAIcon::ArrowPointer => "An arrow pointer, representing a cursor or selection.",
            FAIcon::ArrowRight => "A right arrow, representing forward direction.",
            FAIcon::ArrowRightArrowLeft => {
                "Arrows pointing right and left, representing bidirectional movement."
            }
            FAIcon::ArrowRightFromBracket => "A right arrow coming from a bracket, indicating exit.",
            FAIcon::ArrowRightLong => {
                "A long arrow pointing right, indicating extended forward direction."
            }
            FAIcon::ArrowRightToBracket => "A right arrow pointing to a bracket, indicating entry.",
            FAIcon::ArrowRightToCity => {
                "A right arrow pointing to a city, representing urban direction."
            }
            FAIcon::ArrowRotateLeft => {
                "An arrow rotating to the left, indicating undo or backward movement."
            }
            FAIcon::ArrowRotateRight => {
                "An arrow rotating to the right, indicating redo or forward movement."
            }
            FAIcon::ArrowTrendDown => "A downward trending arrow, representing decline.",
            FAIcon::ArrowTrendUp => "An upward trending arrow, representing growth or increase.",
            FAIcon::ArrowTurnDown => "An arrow turning down, representing downward movement.",
            FAIcon::ArrowTurnUp => "An arrow turning up, representing upward movement.",
            FAIcon::ArrowUp => "An upward arrow, indicating upward direction or increase.",
            FAIcon::ArrowUp19 => "An arrow pointing up with numbers 1 to 9, representing sorting.",
            FAIcon::ArrowUp91 => {
                "An arrow pointing up with numbers 9 to 1, representing reverse sorting."
            }
            FAIcon::ArrowUpAZ => {
                "An arrow pointing up with letters A to Z, representing sorting in alphabetical order."
            }
            FAIcon::ArrowUpFromBracket => {
                "A bracket with an upward arrow, indicating upload or elevation."
            }
            FAIcon::ArrowUpFromGroundWater => {
                "A ground water pump with an arrow pointing up, indicating water extraction."
            }
            FAIcon::ArrowUpFromWaterPump => {
                "A water pump with an arrow pointing up, indicating water extraction."
            }
            FAIcon::ArrowUpLong => "A long arrow pointing up, indicating upward direction.",
            FAIcon::ArrowUpRightDots => {
                "An arrow pointing up and right with dots, representing movement or progression."
            }
            FAIcon::ArrowUpRightFromSquare => {
                "An arrow pointing up-right from a square, representing an external link."
            }
            FAIcon::ArrowUpShortWide => {
                "A short wide arrow pointing up, indicating sorting from smallest to largest."
            }
            FAIcon::ArrowUpWideShort => {
                "A short wide arrow pointing up, indicating sorting from largest to smallest."
            }
            FAIcon::ArrowUpZA => {
                "An arrow pointing up with letters Z to A, representing reverse alphabetical order."
            }
            FAIcon::ArrowsDownToLine => {
                "Arrows pointing down to a line, representing downward movement."
            }
            FAIcon::ArrowsDownToPeople => {
                "Arrows pointing down to people, representing distribution or allocation."
            }
            FAIcon::ArrowsLeftRight => {
                "Arrows pointing left and right, representing bidirectional movement."
            }
            FAIcon::ArrowsLeftRightToLine => {
                "Arrows pointing left and right to a line, representing directional alignment."
            }
            FAIcon::ArrowsRotate => "Rotating arrows, representing refresh or rotation.",
            FAIcon::ArrowsSpin => "Arrows in a spinning motion, representing rotation or refresh.",
            FAIcon::ArrowsSplitUpAndLeft => "Arrows splitting up and left, representing divergence.",
            FAIcon::ArrowsToCircle => "Arrows pointing to a circle, representing centralization.",
            FAIcon::ArrowsToDot => "Arrows pointing to a dot, representing convergence or focus.",
            FAIcon::ArrowsToEye => "Arrows pointing to an eye, representing focus or attention.",
            FAIcon::ArrowsTurnRight => {
                "A set of arrows turning right, representing a directional change."
            }
            FAIcon::ArrowsTurnToDots => "Arrows turning to dots, representing conversion or focus.",
            FAIcon::ArrowsUpDown => {
                "Arrows pointing up and down, representing bidirectional movement."
            }
            FAIcon::ArrowsUpDownLeftRight => {
                "Arrows pointing in all directions, indicating movement or navigation."
            }
            FAIcon::ArrowsUpToLine => "Arrows pointing up to a line, indicating upward movement.",
            FAIcon::Asterisk => "An asterisk, representing additional information or footnotes.",
            FAIcon::At => "The at symbol (@), representing email or social media.",
            FAIcon::Atom => "An atom, representing science or physics.",
            FAIcon::AudioDescription => {
                "A screen with sound waves, indicating audio description for the visually impaired."
            }
            FAIcon::AustralSign => "The symbol for the Argentine austral, indicating currency.",
            FAIcon::Award => "A medal, representing achievement or recognition.",
            FAIcon::B => "The letter \"B\", representing the alphabet.",
            FAIcon::Baby => "A baby face, representing an infant.",
            FAIcon::BabyCarriage => "A baby carriage, representing childcare or infancy.",
            FAIcon::Backward => "An arrow pointing left, indicating backward or rewind.",
            FAIcon::BackwardFast => "Fast backward arrows, representing rapid reverse.",
            FAIcon::BackwardStep => "A step backward symbol, representing reverse or undo.",
            FAIcon::Bacon => "A strip of bacon, representing food or breakfast.",
            FAIcon::Bacteria => "Multiple bacteria, representing microbiology.",
            FAIcon::Bacterium => "A bacterium, representing microbiology.",
            FAIcon::BagShopping => "A shopping bag, representing commerce or shopping.",
            FAIcon::Bahai => "The symbol for the Bahá'í Faith, representing the religion.",
            FAIcon::BahtSign => "The symbol for the Thai baht, indicating currency.",
            FAIcon::Ban => "A circle with a slash, indicating prohibition.",
            FAIcon::BanSmoking => "A cigarette with a ban symbol, representing no smoking.",
            FAIcon::Bandage => "A bandage, representing first aid or healing.",
            FAIcon::BangladeshiTakaSign => {
                "The symbol for the Bangladeshi taka, indicating currency."
            }
            FAIcon::Barcode => "A barcode, representing scanning or product identification.",
            FAIcon::Bars => "Three horizontal bars, indicating a menu or list.",
            FAIcon::BarsProgress => "Bars showing progress, representing loading or progression.",
            FAIcon::BarsStaggered => "Staggered bars, representing a progress indicator.",
            FAIcon::Baseball => "A baseball, representing the sport.",
            FAIcon::BaseballBatBall => "A baseball bat and ball, representing the sport.",
            FAIcon::BasketShopping => "A shopping basket, representing retail or groceries.",
            FAIcon::Basketball => "A basketball, representing the sport.",
            FAIcon::Bath => "A bathtub, representing bathing or bathrooms.",
            FAIcon::BatteryEmpty => "An empty battery, representing no power.",
            FAIcon::BatteryFull => "A full battery, representing full charge or power.",
            FAIcon::BatteryHalf => "A battery half full, representing moderate power.",
            FAIcon::BatteryQuarter => "A battery one-quarter full, representing low power.",
            FAIcon::BatteryThreeQuarters => "A battery three-quarters full, representing power.",
            FAIcon::Bed => "A bed, representing sleep or rest.",
            FAIcon::BedPulse => "A bed with a pulse line, representing healthcare or emergency.",
            FAIcon::BeerMugEmpty => "An empty beer mug, representing beverages.",
            FAIcon::Bell => "A ringing bell, indicating notifications or alerts.",
            FAIcon::BellConcierge => "A concierge bell, representing service or assistance.",
            FAIcon::BellSlash => "A bell with a slash, indicating no notifications.",
            FAIcon::BezierCurve => "A Bézier curve, representing vector graphics or design.",
            FAIcon::Bicycle => "An icon of a bicycle, representing cycling.",
            FAIcon::Binoculars => "A pair of binoculars, indicating search or exploration.",
            FAIcon::Biohazard => "A biohazard symbol, representing hazardous materials.",
            FAIcon::Bitcoin => {
                "The logo of Bitcoin, representing the cryptocurrency, with a black background."
            }
            FAIcon::BitcoinSign => "The symbol for Bitcoin, indicating cryptocurrency.",
            FAIcon::Blender => "A blender, representing kitchen appliances.",
            FAIcon::BlenderPhone => "A blender with a phone, representing multitasking or devices.",
            FAIcon::Blog => "A blog symbol, representing blogging or writing.",
            FAIcon::Bluetooth => "The logo of Bluetooth, representing the wireless technology.",
            FAIcon::BluetoothB => "The logo of Bluetooth B, representing the wireless technology.",
            FAIcon::Bold => "A bold 'B', representing bold text.",
            FAIcon::Bolt => "A lightning bolt, representing speed or electricity.",
            FAIcon::BoltLightning => "A lightning bolt, representing electricity or energy.",
            FAIcon::Bomb => "An icon of a bomb, representing danger or explosive action.",
            FAIcon::Bone => "A bone, representing the skeletal system or pet treats.",
            FAIcon::Bong => "A bong, representing smoking or cannabis use.",
            FAIcon::Book => "A book, representing reading or literature.",
            FAIcon::BookAtlas => "A book with maps, representing an atlas or geography.",
            FAIcon::BookBible => "A book representing the Bible, a holy book in Christianity.",
            FAIcon::BookBookmark => "A book with a bookmark, representing reading or saved pages.",
            FAIcon::BookJournalWhills => {
                "A book representing the Journal of the Whills from Star Wars."
            }
            FAIcon::BookMedical => "A medical book, representing healthcare knowledge.",
            FAIcon::BookOpen => "An open book, representing reading or literature.",
            FAIcon::BookOpenReader => {
                "An open book with a user icon, representing reading or studying."
            }
            FAIcon::BookQuran => "A book representing the Quran, a holy book in Islam.",
            FAIcon::BookSkull => "A book with a skull, representing danger or mystery.",
            FAIcon::BookTanakh => {
                "A book representing the Tanakh, a canonical collection in Judaism."
            }
            FAIcon::Bookmark => "A bookmark, indicating saved items or favorites.",
            FAIcon::BorderAll => "An icon representing all borders.",
            FAIcon::BorderNone => "A border with no lines, indicating no borders.",
            FAIcon::BorderTopLeft => "An icon representing the top-left border.",
            FAIcon::BoreHole => "A borehole, representing drilling or wells.",
            FAIcon::BottleDroplet => "A bottle with a droplet, representing liquid or moisture.",
            FAIcon::BottleWater => "A bottle of water, representing hydration.",
            FAIcon::BowlFood => "A bowl of food, representing dining.",
            FAIcon::BowlRice => "A bowl of rice, representing food.",
            FAIcon::BowlingBall => "A bowling ball, representing the sport.",
            FAIcon::Box => "A simple box, representing a container.",
            FAIcon::BoxArchive => "A box with files, representing storage or archiving.",
            FAIcon::BoxOpen => "A box that is open, representing delivery or unboxing.",
            FAIcon::BoxTissue => "A box of tissues, representing healthcare or hygiene.",
            FAIcon::BoxesPacking => "Packing boxes, representing moving or storage.",
            FAIcon::BoxesStacked => "Stacked boxes, representing storage or organization.",
            FAIcon::Braille => "Braille text, representing accessibility for the blind.",
            FAIcon::Brain => "A brain, representing intelligence or mental processes.",
            FAIcon::BrazilianRealSign => "The symbol for the Brazilian real, indicating currency.",
            FAIcon::BreadSlice => "A slice of bread, representing food.",
            FAIcon::Bridge => "A simple bridge, representing infrastructure.",
            FAIcon::BridgeCircleCheck => {
                "A bridge with a circled check mark, indicating an approved bridge."
            }
            FAIcon::BridgeCircleExclamation => {
                "A bridge with a circled exclamation mark, indicating a bridge with caution."
            }
            FAIcon::BridgeCircleXmark => "A bridge with a circled `X`, indicating a closed bridge.",
            FAIcon::BridgeLock => "A bridge with a lock, representing security.",
            FAIcon::BridgeWater => "A bridge over water, representing infrastructure.",
            FAIcon::Briefcase => "A briefcase, representing work or business.",
            FAIcon::BriefcaseMedical => {
                "A briefcase with a medical cross, representing medical supplies."
            }
            FAIcon::Broom => "A broom, representing cleaning.",
            FAIcon::BroomBall => "A broom with a ball, representing cleaning or a sport.",
            FAIcon::Brush => "A brush, representing painting or art.",
            FAIcon::Btc => "The logo of Bitcoin, representing the cryptocurrency.",
            FAIcon::Bucket => "A bucket, representing a container for liquids.",
            FAIcon::Bug => "A bug, representing an insect or an error in software.",
            FAIcon::BugSlash => "A bug with a slash, indicating no bugs.",
            FAIcon::Bugs => "Multiple bugs, representing software issues or pests.",
            FAIcon::Building => "A tall building, indicating construction or urban areas.",
            FAIcon::BuildingCircleArrowRight => {
                "A building with a circled arrow pointing right, representing a building exit."
            }
            FAIcon::BuildingCircleCheck => {
                "A building with a circled check mark, representing an approved building."
            }
            FAIcon::BuildingCircleExclamation => {
                "A building with a circled exclamation mark, representing a building with caution."
            }
            FAIcon::BuildingCircleXmark => {
                "A building with a circled `X`, representing a closed building."
            }
            FAIcon::BuildingColumns => {
                "A building with columns, representing classical architecture."
            }
            FAIcon::BuildingFlag => "A building with a flag, representing government or institution.",
            FAIcon::BuildingLock => "A building with a lock, representing security.",
            FAIcon::BuildingNgo => {
                "A building with 'NGO', representing a non-governmental organization."
            }
            FAIcon::BuildingShield => "A building with a shield, representing security.",
            FAIcon::BuildingUn => "A building with 'UN', representing the United Nations.",
            FAIcon::BuildingUser => {
                "A building with a user icon, representing a workplace or office."
            }
            FAIcon::BuildingWheat => {
                "A building with wheat, representing agriculture or agribusiness."
            }
            FAIcon::Bullhorn => "A bullhorn, representing announcements or public address.",
            FAIcon::Bullseye => "A bullseye, representing a target or goal.",
            FAIcon::Burger => "A burger, representing food or fast food.",
            FAIcon::Burst => "An explosion or burst, representing impact or energy.",
            FAIcon::Bus => "A bus, representing public transportation.",
            FAIcon::BusSimple => "A simple bus, representing public transportation.",
            FAIcon::BusinessTime => {
                "A briefcase with a clock, representing business hours or time management."
            }
            FAIcon::C => "A capital letter 'C', representing the letter.",
            FAIcon::CableCar => "A cable car, representing a type of public transportation.",
            FAIcon::CakeCandles => "A cake with candles, representing celebration or birthday.",
            FAIcon::Calculator => "A calculator, representing mathematical calculations.",
            FAIcon::Calendar => "A simple calendar, representing scheduling.",
            FAIcon::CalendarCheck => "A calendar with a check mark, representing a confirmed date.",
            FAIcon::CalendarDay => "A calendar showing a day, representing scheduling.",
            FAIcon::CalendarDays => "A calendar with marked days, indicating a schedule or event.",
            FAIcon::CalendarMinus => "A calendar with a minus sign, representing removing an event.",
            FAIcon::CalendarPlus => "A calendar with a plus sign, representing adding an event.",
            FAIcon::CalendarWeek => "A calendar with a week view, representing weekly schedule.",
            FAIcon::CalendarXmark => "A calendar with an `X`, representing a cancelled date.",
            FAIcon::Camera => "An icon of a camera, representing photography.",
            FAIcon::CameraRetro => "An old-fashioned camera, indicating photography or photos.",
            FAIcon::CameraRotate => "A camera with a rotation arrow, representing photo orientation.",
            FAIcon::Campground => "A campground symbol, representing camping or outdoor activities.",
            FAIcon::CandyCane => "A candy cane, representing Christmas or sweets.",
            FAIcon::Cannabis => "A cannabis leaf, representing the plant or its products.",
            FAIcon::Capsules => "Two capsules, representing medication or supplements.",
            FAIcon::Car => "An icon of a car, indicating a vehicle or transportation.",
            FAIcon::CarBattery => "A car battery, representing automotive power.",
            FAIcon::CarBurst => "A car with a burst, indicating accident or impact.",
            FAIcon::CarOn => "A car with a key, indicating vehicle status.",
            FAIcon::CarRear => "The rear view of a car, representing transportation.",
            FAIcon::CarSide => "A side view of a car, indicating transportation.",
            FAIcon::CarTunnel => "A car in a tunnel, representing travel or transportation.",
            FAIcon::Caravan => "A caravan, representing travel or transportation.",
            FAIcon::CaretDown => "A downward caret, representing dropdowns or more options.",
            FAIcon::CaretLeft => "A caret pointing left, indicating backward direction.",
            FAIcon::CaretRight => "A caret pointing right, indicating forward direction.",
            FAIcon::CaretUp => "An upward pointing caret, indicating expansion or scroll up.",
            FAIcon::Carrot => "A carrot, representing the vegetable.",
            FAIcon::CartArrowDown => {
                "A shopping cart with a downward arrow, representing adding to cart."
            }
            FAIcon::CartFlatbed => "A flatbed cart, representing transportation or logistics.",
            FAIcon::CartFlatbedSuitcase => {
                "A flatbed cart with a suitcase, representing luggage transport."
            }
            FAIcon::CartPlus => "A shopping cart with a plus sign, representing adding to cart.",
            FAIcon::CartShopping => "A shopping cart, representing commerce or shopping.",
            FAIcon::CashRegister => "A cash register, indicating point of sale or retail.",
            FAIcon::Cat => "A cat, representing the animal.",
            FAIcon::CcAmazonPay => {
                "The logo of CC Amazon Pay, representing the credit card payment service."
            }
            FAIcon::CcAmex => "The logo of CC Amex, representing the credit card payment service.",
            FAIcon::CcApplePay => {
                "The logo of CC Apple Pay, representing the credit card payment service."
            }
            FAIcon::CcDinersClub => {
                "The logo of CC Diners Club, representing the credit card payment service."
            }
            FAIcon::CcDiscover => {
                "The logo of CC Discover, representing the credit card payment service."
            }
            FAIcon::CcJcb => "The logo of CC JCB, representing the credit card payment service.",
            FAIcon::CcMastercard => "The logo of MasterCard, indicating a credit card or payment.",
            FAIcon::CcPaypal => {
                "The logo of CC PayPal, representing the credit card payment service."
            }
            FAIcon::CcStripe => {
                "The logo of CC Stripe, representing the credit card payment service."
            }
            FAIcon::CcVisa => "The logo of Visa credit card, indicating payment.",
            FAIcon::CediSign => "The symbol for the Ghanaian cedi, indicating currency.",
            FAIcon::CentSign => "The symbol for cent, indicating currency.",
            FAIcon::Certificate => "A certificate, indicating achievement or certification.",
            FAIcon::Chair => "A chair, representing seating or furniture.",
            FAIcon::Chalkboard => "A chalkboard, representing teaching or education.",
            FAIcon::ChalkboardUser => {
                "A chalkboard with a user icon, representing teaching or instruction."
            }
            FAIcon::ChampagneGlasses => "Two champagne glasses clinking, representing celebration.",
            FAIcon::ChargingStation => "A charging station, representing electric vehicle charging.",
            FAIcon::ChartArea => "An area chart, representing data trends.",
            FAIcon::ChartBar => "A bar chart, representing data comparison.",
            FAIcon::ChartColumn => "A column chart, representing data visualization.",
            FAIcon::ChartGantt => "A Gantt chart, representing project management.",
            FAIcon::ChartLine => "A line chart, representing data trends.",
            FAIcon::ChartPie => "A pie chart, representing data visualization.",
            FAIcon::ChartSimple => "A simple bar chart, representing data or statistics.",
            FAIcon::Check => "A check mark, symbolizing confirmation or success.",
            FAIcon::CheckDouble => "A double check mark, representing confirmation or approval.",
            FAIcon::CheckToSlot => "A checkmark entering a slot, representing verification.",
            FAIcon::Cheese => "A wedge of cheese, representing dairy or food.",
            FAIcon::Chess => "A chess piece, representing the game of chess.",
            FAIcon::ChessBishop => "A chess bishop, representing the game of chess.",
            FAIcon::ChessBoard => "A chess board, representing the game of chess.",
            FAIcon::ChessKing => "A chess king, representing the game of chess.",
            FAIcon::ChessKnight => "A chess knight, representing the game of chess.",
            FAIcon::ChessPawn => "A chess pawn, representing the game of chess.",
            FAIcon::ChessQueen => "A chess queen, representing the game of chess.",
            FAIcon::ChessRook => "A chess rook, representing the game of chess.",
            FAIcon::ChevronDown => "A downward chevron, representing a dropdown or more options.",
            FAIcon::ChevronLeft => "A chevron pointing left, indicating backward direction.",
            FAIcon::ChevronRight => "A chevron pointing right, indicating forward direction.",
            FAIcon::ChevronUp => {
                "A chevron pointing upwards, indicating upward movement or navigation."
            }
            FAIcon::Child => "A child, indicating a young person.",
            FAIcon::ChildCombatant => "A child holding a weapon, representing child soldiers.",
            FAIcon::ChildDress => "A child in a dress, representing a young girl.",
            FAIcon::ChildReaching => "A child reaching out, representing assistance or curiosity.",
            FAIcon::Children => "Two children, representing youth or family.",
            FAIcon::Church => "A church building, representing a place of worship.",
            FAIcon::Circle => "A simple circle, representing shape or completeness.",
            FAIcon::CircleArrowDown => {
                "A circle with an arrow pointing down, indicating downward movement."
            }
            FAIcon::CircleArrowLeft => {
                "A circle with an arrow pointing left, indicating backward movement."
            }
            FAIcon::CircleArrowRight => {
                "A circle with an arrow pointing right, indicating forward movement."
            }
            FAIcon::CircleArrowUp => {
                "A circle with an arrow pointing up, indicating upward movement."
            }
            FAIcon::CircleCheck => "A check mark inside a circle, indicating confirmation.",
            FAIcon::CircleChevronDown => {
                "A circle with a chevron pointing down, indicating downward direction."
            }
            FAIcon::CircleChevronLeft => {
                "A circle with a chevron pointing left, indicating backward direction."
            }
            FAIcon::CircleChevronRight => {
                "A circle with a chevron pointing right, indicating forward direction."
            }
            FAIcon::CircleChevronUp => {
                "A circle with a chevron pointing up, indicating upward direction."
            }
            FAIcon::CircleDollarToSlot => "A circle with a dollar sign and slot, indicating payment.",
            FAIcon::CircleDot => "A circle with a dot, indicating focus or selection.",
            FAIcon::CircleDown => {
                "A downward arrow inside a circle, indicating scroll down or download."
            }
            FAIcon::CircleExclamation => {
                "An exclamation mark inside a circle, indicating important information or alerts."
            }
            FAIcon::CircleH => "A circle with an 'H', representing hospital.",
            FAIcon::CircleHalfStroke => {
                "A half-filled circle, representing partial loading or status."
            }
            FAIcon::CircleInfo => "A circle with an 'i', representing information.",
            FAIcon::CircleLeft => "A circle with a left arrow, indicating backward direction.",
            FAIcon::CircleMinus => "A circle with a minus sign, indicating subtraction.",
            FAIcon::CircleNodes => "A circle with nodes, representing connections or network.",
            FAIcon::CircleNotch => {
                "A circle with a notch, representing a loading or progress indicator."
            }
            FAIcon::CirclePause => "A circle with a pause symbol, indicating media pause.",
            FAIcon::CirclePlay => "A circle with a play symbol, indicating media playback.",
            FAIcon::CirclePlus => "A circle with a plus sign, indicating addition.",
            FAIcon::CircleQuestion => "A circle with a question mark, indicating inquiry.",
            FAIcon::CircleRadiation => "A circle with a radiation symbol, indicating hazard.",
            FAIcon::CircleRight => "A circle with a right arrow, indicating forward direction.",
            FAIcon::CircleStop => "A circle with a stop symbol, indicating cessation.",
            FAIcon::CircleUp => "An upward arrow inside a circle, indicating scroll up or upload.",
            FAIcon::CircleUser => "A user icon inside a circle, indicating a user profile.",
            FAIcon::CircleXmark => "A circled `X` mark, indicating closure or deletion.",
            FAIcon::City => "A skyline of buildings, indicating an urban area or city.",
            FAIcon::Clapperboard => "A clapperboard, representing filmmaking or production.",
            FAIcon::Clipboard => "A clipboard, representing note-taking or data recording.",
            FAIcon::ClipboardCheck => "A clipboard with a check mark, representing completed tasks.",
            FAIcon::ClipboardList => "A clipboard with a list, representing tasks or notes.",
            FAIcon::ClipboardQuestion => {
                "A clipboard with a question mark, representing inquiry or uncertainty."
            }
            FAIcon::ClipboardUser => "A clipboard with a user icon, representing user data or forms.",
            FAIcon::Clock => "A clock face, indicating time.",
            FAIcon::ClockRotateLeft => {
                "A clock with an arrow rotating left, representing time reversal."
            }
            FAIcon::Clone => "Two overlapping squares, indicating duplication or cloning.",
            FAIcon::ClosedCaptioning => {
                "A closed captioning symbol, representing subtitles or accessibility."
            }
            FAIcon::Cloud => "A cloud, representing cloud storage or weather.",
            FAIcon::CloudArrowDown => "A cloud with a downward arrow, representing cloud download.",
            FAIcon::CloudArrowUp => "A cloud with an upward arrow, indicating upload to the cloud.",
            FAIcon::CloudBolt => "A cloud with a lightning bolt, representing a thunderstorm.",
            FAIcon::CloudMeatball => "A cloud with meatballs, representing food or weather.",
            FAIcon::CloudMoon => "A cloud with a moon, representing partly cloudy night.",
            FAIcon::CloudMoonRain => "A cloud with a moon and rain, representing nighttime rain.",
            FAIcon::CloudRain => "A cloud with rain, representing weather or precipitation.",
            FAIcon::CloudShowersHeavy => "A cloud with heavy rain, representing a downpour.",
            FAIcon::CloudShowersWater => "A cloud with water droplets, representing rain.",
            FAIcon::CloudSun => "A cloud with a sun, representing partly cloudy weather.",
            FAIcon::CloudSunRain => "A cloud with sun and rain, representing mixed weather.",
            FAIcon::Cloudflare => {
                "The logo of Cloudflare, representing the web infrastructure company."
            }
            FAIcon::Clover => "A clover, representing luck or St. Patrick's Day.",
            FAIcon::Code => "An icon representing coding or programming.",
            FAIcon::CodeBranch => "A branch in code, indicating version control or branching.",
            FAIcon::CodeCommit => "A check mark, representing a code commit.",
            FAIcon::CodeCompare => {
                "Two pieces of code being compared, indicating code review or comparison."
            }
            FAIcon::CodeFork => "A forked path, representing branching in code.",
            FAIcon::CodeMerge => "A symbol representing code merging.",
            FAIcon::CodePullRequest => "A symbol representing a pull request in code versioning.",
            FAIcon::Codepen => {
                "The logo of CodePen, a social development environment for front-end designers and developers."
            }
            FAIcon::Coins => "Coins, representing money or currency.",
            FAIcon::ColonSign => "A colon symbol, representing punctuation or separation.",
            FAIcon::Comment => "A speech bubble, indicating comments or communication.",
            FAIcon::CommentDollar => {
                "A speech bubble with a dollar sign, representing financial comments."
            }
            FAIcon::CommentDots => {
                "A speech bubble with dots, representing comments or conversation."
            }
            FAIcon::CommentMedical => {
                "A speech bubble with a medical cross, representing medical communication."
            }
            FAIcon::CommentSlash => "A speech bubble with a slash, indicating no comments.",
            FAIcon::CommentSms => "A speech bubble with \"SMS\", representing text messaging.",
            FAIcon::Comments => "Multiple speech bubbles, indicating conversation or comments.",
            FAIcon::CommentsDollar => {
                "A speech bubble with a dollar sign, representing financial discussions."
            }
            FAIcon::CompactDisc => "A compact disc, representing media storage.",
            FAIcon::Compass => "A compass, representing navigation or direction.",
            FAIcon::CompassDrafting => "A drafting compass, representing design or architecture.",
            FAIcon::Compress => "A compress icon, representing minimization.",
            FAIcon::Computer => "A desktop computer, representing computing or technology.",
            FAIcon::ComputerMouse => "A computer mouse, representing input device.",
            FAIcon::Cookie => "A cookie, representing snacks or website tracking.",
            FAIcon::CookieBite => "A bitten cookie, representing snacks or desserts.",
            FAIcon::Copy => "Two overlapping documents, indicating copying.",
            FAIcon::Copyright => "A circled 'C', indicating copyright protection.",
            FAIcon::Couch => "A couch, representing furniture or relaxation.",
            FAIcon::Cow => "A cow, representing the animal.",
            FAIcon::CreditCard => "A credit card, indicating payment or financial transactions.",
            FAIcon::CriticalRole => "The logo of Critical Role, representing the web series.",
            FAIcon::Crop => "An image crop icon, representing editing.",
            FAIcon::CropSimple => "A simple crop icon, representing image cropping.",
            FAIcon::Cross => "A cross, representing religion or medical aid.",
            FAIcon::Crosshairs => "A crosshair, indicating targeting or precision.",
            FAIcon::Crow => "A crow, representing the bird.",
            FAIcon::Crown => "A crown, representing royalty or achievement.",
            FAIcon::Crutch => "A crutch, representing injury support.",
            FAIcon::CruzeiroSign => "The symbol for the Brazilian cruzeiro, indicating currency.",
            FAIcon::Cube => "A 3D cube, representing geometry or structure.",
            FAIcon::Cubes => "Multiple cubes, representing 3D objects.",
            FAIcon::CubesStacked => "Stacked cubes, representing building blocks.",
            FAIcon::D => "A capital letter 'D', representing the letter.",
            FAIcon::DAndD => {
                "The logo of Dungeons & Dragons, representing the tabletop role-playing game."
            }
            FAIcon::DAndDBeyond => {
                "The logo of D&D Beyond, representing the Dungeons & Dragons toolset."
            }
            FAIcon::Database => "A stack of disks, representing a database.",
            FAIcon::DeleteLeft => "An arrow pointing left with a line, representing backspace.",
            FAIcon::Democrat => "The logo of the Democratic Party, representing the political party.",
            FAIcon::Desktop => "A desktop computer, indicating computing or work.",
            FAIcon::Dharmachakra => "The dharma wheel, representing Buddhism.",
            FAIcon::DiagramNext => "A diagram showing the next step, representing progression.",
            FAIcon::DiagramPredecessor => "A diagram showing predecessors, representing planning.",
            FAIcon::DiagramProject => "A diagram showing a project, representing planning.",
            FAIcon::DiagramSuccessor => "A diagram showing successors, representing progression.",
            FAIcon::Diamond => "A diamond, representing luxury or value.",
            FAIcon::DiamondTurnRight => {
                "A diamond turned to the right, representing geometric shapes."
            }
            FAIcon::Dice => "A pair of dice, representing games or chance.",
            FAIcon::DiceD20 => "A 20-sided die, representing tabletop gaming.",
            FAIcon::DiceD6 => "A six-sided die, representing gaming or chance.",
            FAIcon::DiceFive => "Two dice showing five, representing chance or gaming.",
            FAIcon::DiceFour => "Two dice showing four, representing chance or gaming.",
            FAIcon::DiceOne => "Two dice showing one, representing chance or gaming.",
            FAIcon::DiceSix => "Two dice showing six, representing chance or gaming.",
            FAIcon::DiceThree => "Two dice showing three, representing chance or gaming.",
            FAIcon::DiceTwo => "Two dice showing two, representing chance or gaming.",
            FAIcon::Discord => "The logo of Discord, a chat and communication platform for gamers.",
            FAIcon::Disease => "A virus, representing illness.",
            FAIcon::Display => "A computer display, representing screens or monitors.",
            FAIcon::Divide => "A division sign, representing mathematical operations.",
            FAIcon::Dna => "A DNA strand, representing genetics.",
            FAIcon::Docker => "The logo of Docker, a platform for containerized applications.",
            FAIcon::Dog => "A dog, representing the animal.",
            FAIcon::DollarSign => "A dollar sign, indicating currency or money.",
            FAIcon::Dolly => "A dolly, representing transport or moving.",
            FAIcon::DongSign => "The symbol for the Vietnamese dong, indicating currency.",
            FAIcon::DoorClosed => "A closed door, representing privacy or security.",
            FAIcon::DoorOpen => "An open door, indicating entry or exit.",
            FAIcon::Dove => "A dove, representing peace.",
            FAIcon::DownLeftAndUpRightToCenter => {
                "Arrows pointing down-left and up-right to a center, representing convergence."
            }
            FAIcon::DownLong => "A long arrow pointing down, representing downward direction.",
            FAIcon::Download => "A downward arrow, typically used to indicate download actions.",
            FAIcon::Dragon => "A dragon, representing mythical creatures or fantasy.",
            FAIcon::DrawPolygon => "A polygon, representing geometric shapes.",
            FAIcon::Dribbble => "The logo of Dribbble, a platform for showcasing design work.",
            FAIcon::Dropbox => "The logo of Dropbox, a cloud storage service.",
            FAIcon::Droplet => "A droplet of water, representing liquid or fluidity.",
            FAIcon::DropletSlash => "A droplet with a slash, representing no water.",
            FAIcon::Drum => "A drum, representing music.",
            FAIcon::DrumSteelpan => "A steelpan drum, representing music.",
            FAIcon::DrumstickBite => "A drumstick with a bite, representing food.",
            FAIcon::Dumbbell => "A dumbbell, representing fitness or weightlifting.",
            FAIcon::Dumpster => "A dumpster, representing waste disposal.",
            FAIcon::DumpsterFire => "A dumpster on fire, representing chaos or disaster.",
            FAIcon::Dungeon => "A dungeon, representing a prison or game environment.",
            FAIcon::E => "The letter \"E\", representing the alphabet.",
            FAIcon::EarDeaf => "An ear with a slash, representing hearing impairment.",
            FAIcon::EarListen => "An ear with sound waves, representing listening.",
            FAIcon::EarthAfrica => "A globe focusing on Africa, representing global presence.",
            FAIcon::EarthAmericas => "A globe focusing on the Americas, representing global reach.",
            FAIcon::EarthAsia => "A globe focusing on Asia, representing global presence.",
            FAIcon::EarthEurope => "A globe focusing on Europe, representing global presence.",
            FAIcon::EarthOceania => "A globe focusing on Oceania, representing global presence.",
            FAIcon::Egg => "An egg, representing food or Easter.",
            FAIcon::Eject => "An eject button, representing removal.",
            FAIcon::Elevator => "An elevator, representing vertical transportation.",
            FAIcon::Ellipsis => "A horizontal ellipsis, representing more options.",
            FAIcon::EllipsisVertical => "A vertical ellipsis, representing more options.",
            FAIcon::Envelope => "A closed envelope, representing email or messages.",
            FAIcon::EnvelopeCircleCheck => {
                "An envelope with a circled check, representing approved mail."
            }
            FAIcon::EnvelopeOpen => "An open envelope, representing received message.",
            FAIcon::EnvelopeOpenText => "An open envelope with text, representing received message.",
            FAIcon::EnvelopesBulk => "Multiple envelopes, representing bulk mail.",
            FAIcon::Equals => "An equals sign, representing equality.",
            FAIcon::Eraser => "An eraser, representing correction or deletion.",
            FAIcon::Ethereum => "The logo of Ethereum, representing the cryptocurrency.",
            FAIcon::Ethernet => "An Ethernet port, representing network connectivity.",
            FAIcon::EuroSign => "The symbol for the euro, indicating currency.",
            FAIcon::Exclamation => "A large exclamation mark, indicating importance or alerts.",
            FAIcon::Expand => "An outward pointing arrows from a box, indicating expansion.",
            FAIcon::Explosion => "An explosion, representing blast or impact.",
            FAIcon::Eye => "An eye, indicating visibility or views.",
            FAIcon::EyeDropper => "An eyedropper, representing precision or medical use.",
            FAIcon::EyeLowVision => "An eye with low vision, representing visual impairment.",
            FAIcon::EyeSlash => {
                "An eye with a slash through it, indicating hidden or invisible content."
            }
            FAIcon::F => "A capital letter 'F', representing the letter.",
            FAIcon::FaceAngry => "An angry face, representing anger.",
            FAIcon::FaceDizzy => "A dizzy face, representing confusion.",
            FAIcon::FaceFlushed => "A flushed face, representing embarrassment.",
            FAIcon::FaceFrown => "A frowning face, representing sadness.",
            FAIcon::FaceFrownOpen => "A frowning face with open mouth, representing sadness.",
            FAIcon::FaceGrimace => "A grimacing face, representing discomfort.",
            FAIcon::FaceGrin => "A grinning face, representing happiness.",
            FAIcon::FaceGrinBeam => "A grinning face with beams, representing joy.",
            FAIcon::FaceGrinBeamSweat => "A grinning face with beam and sweat, representing relief.",
            FAIcon::FaceGrinHearts => "A grinning face with hearts, representing love.",
            FAIcon::FaceGrinSquint => "A grinning face with squinted eyes, representing humor.",
            FAIcon::FaceGrinSquintTears => {
                "A grinning face with squinting eyes and tears, representing laughter."
            }
            FAIcon::FaceGrinStars => "A grinning face with stars, representing excitement.",
            FAIcon::FaceGrinTears => "A grinning face with tears, representing laughter.",
            FAIcon::FaceGrinTongue => "A grinning face with tongue out, representing playfulness.",
            FAIcon::FaceGrinTongueSquint => {
                "A grinning face with tongue out and squinted eyes, representing silliness."
            }
            FAIcon::FaceGrinTongueWink => {
                "A grinning face with tongue out and wink, representing silliness."
            }
            FAIcon::FaceGrinWide => "A wide grinning face, representing happiness.",
            FAIcon::FaceGrinWink => "A grinning face with a wink, representing playfulness.",
            FAIcon::FaceKiss => "A kissing face, representing affection.",
            FAIcon::FaceKissBeam => "A kissing face with beams, representing love.",
            FAIcon::FaceKissWinkHeart => {
                "A kissing face with a wink and heart, representing affection."
            }
            FAIcon::FaceLaugh => "A laughing face, representing humor.",
            FAIcon::FaceLaughBeam => "A laughing face with beams, representing joy.",
            FAIcon::FaceLaughSquint => "A laughing face with squinted eyes, representing humor.",
            FAIcon::FaceLaughWink => "A laughing face with a wink, representing humor.",
            FAIcon::FaceMeh => "A meh face, representing indifference.",
            FAIcon::FaceMehBlank => "A blank face, representing indifference.",
            FAIcon::FaceRollingEyes => "A face with rolling eyes, representing annoyance.",
            FAIcon::FaceSadCry => "A crying face, representing sadness or crying.",
            FAIcon::FaceSadTear => "A sad face with a tear, representing sadness or crying.",
            FAIcon::FaceSmile => "A smiling face, indicating happiness or friendliness.",
            FAIcon::FaceSmileBeam => "A smiling face with beams, representing joy.",
            FAIcon::FaceSmileWink => {
                "A smiling face with a wink, representing happiness or playfulness."
            }
            FAIcon::FaceSurprise => "A surprised face, representing shock.",
            FAIcon::FaceTired => "A tired face, representing fatigue.",
            FAIcon::Facebook => "The logo of Facebook, representing the social media platform.",
            FAIcon::Fan => "A fan, representing cooling or ventilation.",
            FAIcon::FantasyFlightGames => {
                "The logo of Fantasy Flight Games, representing the game publisher."
            }
            FAIcon::Faucet => "A faucet, representing plumbing or water.",
            FAIcon::FaucetDrip => "A faucet with a drip, representing water or plumbing.",
            FAIcon::Fax => "A fax machine, representing facsimile transmission.",
            FAIcon::Feather => "A feather, indicating lightness or writing.",
            FAIcon::FeatherPointed => "A pointed feather, representing writing or quill.",
            FAIcon::Ferry => "A ferry boat, representing water transportation.",
            FAIcon::Figma => "The logo of Figma, a design and prototyping tool.",
            FAIcon::File => "A simple document, indicating a file or document.",
            FAIcon::FileArrowDown => "A file with a downward arrow, representing file download.",
            FAIcon::FileArrowUp => "A file with an upward arrow, representing file upload.",
            FAIcon::FileAudio => "A file with an audio symbol, representing audio files.",
            FAIcon::FileCircleCheck => "A file with a circled check, representing approved files.",
            FAIcon::FileCircleExclamation => {
                "A file with a circled exclamation mark, representing important files."
            }
            FAIcon::FileCircleMinus => "A file with a circled minus, representing removed files.",
            FAIcon::FileCirclePlus => "A file with a circled plus, representing added files.",
            FAIcon::FileCircleQuestion => {
                "A file with a circled question mark, representing unknown files."
            }
            FAIcon::FileCircleXmark => "A file with a circled `X`, representing deleted files.",
            FAIcon::FileCode => "A file with code, representing programming files.",
            FAIcon::FileContract => {
                "A document with a signature line, indicating a contract or agreement."
            }
            FAIcon::FileCsv => "A file with CSV text, representing CSV files.",
            FAIcon::FileExcel => "A document with the Excel logo, indicating a spreadsheet file.",
            FAIcon::FileExport => "A document with an arrow pointing out, indicating file export.",
            FAIcon::FileImage => "A file with an image, representing image files.",
            FAIcon::FileImport => "A document with an arrow pointing in, indicating file import.",
            FAIcon::FileInvoice => "A document with an invoice, indicating billing or payments.",
            FAIcon::FileInvoiceDollar => {
                "A file with a dollar sign, representing financial documents."
            }
            FAIcon::FileLines => "A file with lines, representing documents.",
            FAIcon::FileMedical => "A file with a medical symbol, representing medical records.",
            FAIcon::FilePdf => "A file with a PDF symbol, representing a document.",
            FAIcon::FilePen => "A file with a pen, representing editable documents.",
            FAIcon::FilePowerpoint => "A file with a PowerPoint symbol, representing presentations.",
            FAIcon::FilePrescription => {
                "A file with a prescription symbol, representing medical records."
            }
            FAIcon::FileShield => "A file with a shield, representing secure documents.",
            FAIcon::FileSignature => "A file with a signature, representing signed documents.",
            FAIcon::FileVideo => "A file with a video symbol, representing video files.",
            FAIcon::FileWaveform => "A file with a waveform, representing audio files.",
            FAIcon::FileWord => "A file with a Word symbol, representing a document.",
            FAIcon::FileZipper => "A file with a zipper, representing compressed files.",
            FAIcon::Fill => "A paint bucket pouring, representing filling or color.",
            FAIcon::FillDrip => "A paint bucket dripping, representing paint or color fill.",
            FAIcon::Film => "A strip of film, representing movies or filming.",
            FAIcon::Filter => "A funnel filter, representing filtration or sorting.",
            FAIcon::FilterCircleDollar => {
                "A filter with a circled dollar sign, representing financial filtering."
            }
            FAIcon::FilterCircleXmark => "A filter with a circled X, representing filter removal.",
            FAIcon::Fingerprint => "A fingerprint, representing identity or security.",
            FAIcon::Fire => "A flame, representing fire or heat.",
            FAIcon::FireBurner => "A fire burner, representing heating or cooking.",
            FAIcon::FireExtinguisher => "A fire extinguisher, representing safety equipment.",
            FAIcon::FireFlameCurved => "A curved flame, representing fire.",
            FAIcon::FireFlameSimple => "A simple flame, representing fire or heat.",
            FAIcon::Fish => "A fish, representing the animal or aquatic life.",
            FAIcon::FishFins => "A fish with fins, representing the animal or swimming.",
            FAIcon::Flag => "A flag, indicating a nation or marking something important.",
            FAIcon::FlagCheckered => "A checkered flag, representing racing or completion.",
            FAIcon::FlagUsa => "The flag of the USA, representing the United States of America.",
            FAIcon::Flask => "A laboratory flask, representing science or experimentation.",
            FAIcon::FlaskVial => "A flask and vial, representing science or experimentation.",
            FAIcon::FloppyDisk => "A floppy disk, representing data storage.",
            FAIcon::FlorinSign => "The symbol for the florin, indicating currency.",
            FAIcon::Folder => "A folder, representing a collection of documents or files.",
            FAIcon::FolderClosed => "A closed folder, representing file storage.",
            FAIcon::FolderMinus => "A folder with a minus sign, indicating removing files.",
            FAIcon::FolderOpen => "An open folder, indicating accessible files or documents.",
            FAIcon::FolderPlus => "A folder with a plus sign, indicating adding files.",
            FAIcon::FolderTree => "A folder with a tree structure, representing organization.",
            FAIcon::Font => "A capital letter 'A', representing typography or fonts.",
            FAIcon::FontAwesome => "The logo of Font Awesome, representing the icon set.",
            FAIcon::Football => "A football, representing the sport.",
            FAIcon::Forward => "An arrow pointing right, indicating forward or next.",
            FAIcon::ForwardFast => "Two arrows pointing forward, representing fast forward.",
            FAIcon::ForwardStep => {
                "An arrow pointing forward with a vertical line, indicating step forward."
            }
            FAIcon::FrancSign => "The symbol for the franc, indicating currency.",
            FAIcon::Frog => "A frog, representing the animal.",
            FAIcon::Futbol => "A soccer ball, representing the sport of soccer.",
            FAIcon::G => "A capital letter 'G', representing the letter.",
            FAIcon::GalacticRepublic => {
                "The logo of the Galactic Republic, representing the Star Wars faction."
            }
            FAIcon::GalacticSenate => {
                "The logo of the Galactic Senate, representing the Star Wars faction."
            }
            FAIcon::Gamepad => "A video game controller, representing gaming.",
            FAIcon::GasPump => "A gas pump, representing fuel or energy.",
            FAIcon::Gauge => "A speedometer or gauge, representing measurement or speed.",
            FAIcon::GaugeHigh => "A high gauge, representing high level or measurement.",
            FAIcon::GaugeSimple => "A simple gauge, representing measurement.",
            FAIcon::GaugeSimpleHigh => "A gauge with a high reading, representing high measurement.",
            FAIcon::Gavel => "A gavel, representing law or auctions.",
            FAIcon::Gear => "A gear, representing settings or machinery.",
            FAIcon::Gears => "Multiple gears, representing settings or machinery.",
            FAIcon::Gem => "A gemstone, representing jewelry or value.",
            FAIcon::Genderless => "A genderless symbol, representing gender neutrality.",
            FAIcon::Gg => "The logo of GG, representing good game.",
            FAIcon::GgCircle => "A circle with \"GG\", representing good game.",
            FAIcon::Ghost => "A ghost, often used for spooky or playful themes.",
            FAIcon::Gift => "A wrapped gift box, representing presents or surprises.",
            FAIcon::Gifts => "Two wrapped gifts, representing presents or surprises.",
            FAIcon::Github => "The logo of GitHub, representing the code hosting platform.",
            FAIcon::GlassWater => "A glass of water, representing hydration.",
            FAIcon::GlassWaterDroplet => "A glass of water with a droplet, representing hydration.",
            FAIcon::Glasses => "A pair of glasses, representing vision or eyewear.",
            FAIcon::Globe => "A globe, representing the world or global reach.",
            FAIcon::GolfBallTee => "A golf ball on a tee, representing golf.",
            FAIcon::Google => "The logo of Google, a popular search engine.",
            FAIcon::GooglePay => "The logo of Google Pay, representing the payment service.",
            FAIcon::GoogleWallet => "The logo of Google Wallet, representing the payment service.",
            FAIcon::Gopuram => "A gopuram, representing a Hindu temple tower.",
            FAIcon::GraduationCap => "A graduation cap, representing education or graduation.",
            FAIcon::GreaterThan => "A greater than sign, representing mathematical operations.",
            FAIcon::GreaterThanEqual => {
                "A greater than or equal sign, representing mathematical operations."
            }
            FAIcon::Grip => "Dots indicating grip or draggable interface.",
            FAIcon::GripLines => "Horizontal lines indicating grip or draggable interface.",
            FAIcon::GripLinesVertical => "Vertical lines indicating grip or draggable interface.",
            FAIcon::GripVertical => "Vertical dots indicating grip or draggable interface.",
            FAIcon::GroupArrowsRotate => {
                "Multiple arrows rotating around a group, indicating movement or rotation."
            }
            FAIcon::GuaraniSign => "The symbol for the Paraguayan guaraní, indicating currency.",
            FAIcon::Guitar => "A guitar, representing music.",
            FAIcon::Gun => "A gun, representing firearms.",
            FAIcon::H => "A capital letter 'H', representing the letter.",
            FAIcon::Hammer => "A hammer, representing tools or construction.",
            FAIcon::Hamsa => "A hamsa hand, representing protection or luck.",
            FAIcon::Hand => "A raised hand, indicating a stop or request for attention.",
            FAIcon::HandBackFist => "A hand with a fist facing backwards, representing strength.",
            FAIcon::HandDots => "A hand with dots, representing tactile or touch.",
            FAIcon::HandFist => "A fist, representing strength or power.",
            FAIcon::HandHolding => "A hand holding something, representing support.",
            FAIcon::HandHoldingDollar => {
                "A hand holding a dollar sign, representing financial support."
            }
            FAIcon::HandHoldingDroplet => "A hand holding a droplet, representing water or liquid.",
            FAIcon::HandHoldingHand => {
                "A hand holding another hand, representing support or assistance."
            }
            FAIcon::HandHoldingHeart => "A hand holding a heart, symbolizing charity or care.",
            FAIcon::HandHoldingMedical => {
                "A hand holding a medical symbol, representing healthcare or support."
            }
            FAIcon::HandLizard => "A hand making a lizard gesture, representing the animal.",
            FAIcon::HandMiddleFinger => {
                "A hand making the middle finger gesture, representing rudeness."
            }
            FAIcon::HandPeace => "A hand making a peace sign, representing peace or victory.",
            FAIcon::HandPointDown => "A hand pointing down, representing direction.",
            FAIcon::HandPointLeft => "A hand pointing to the left, representing direction.",
            FAIcon::HandPointRight => "A hand pointing to the right, representing direction.",
            FAIcon::HandPointUp => "A hand pointing upwards, indicating direction or emphasis.",
            FAIcon::HandPointer => "A hand pointer, representing selection or clicking.",
            FAIcon::HandScissors => {
                "A hand making a scissors gesture, representing the game rock-paper-scissors."
            }
            FAIcon::HandSparkles => "A hand with sparkles, representing magic or cleanliness.",
            FAIcon::HandSpock => "A hand making the Vulcan salute, representing Star Trek.",
            FAIcon::Handcuffs => "A pair of handcuffs, representing law enforcement or restraint.",
            FAIcon::Hands => "Two hands, indicating help or collaboration.",
            FAIcon::HandsAslInterpreting => "Hands signing in ASL, representing sign language.",
            FAIcon::HandsBound => {
                "A pair of hands bound together, representing restraint or solidarity."
            }
            FAIcon::HandsBubbles => "Hands with bubbles, representing washing or cleanliness.",
            FAIcon::HandsClapping => "Hands clapping, representing applause or appreciation.",
            FAIcon::HandsHolding => {
                "A pair of hands holding something, representing support or unity."
            }
            FAIcon::HandsHoldingChild => {
                "A pair of hands holding a child, representing care or protection."
            }
            FAIcon::HandsHoldingCircle => {
                "A pair of hands holding a circle, representing support or unity."
            }
            FAIcon::HandsPraying => "Hands in a praying position, representing prayer or hope.",
            FAIcon::Handshake => "Two hands shaking, indicating agreement or partnership.",
            FAIcon::HandshakeAngle => {
                "A handshake at an angle, representing agreement or partnership."
            }
            FAIcon::HandshakeSimple => "A simple handshake, representing agreement or partnership.",
            FAIcon::HandshakeSimpleSlash => "A handshake with a slash, indicating no agreement.",
            FAIcon::HandshakeSlash => {
                "Two hands shaking with a slash through them, indicating no agreement or social distancing."
            }
            FAIcon::Hanukiah => "A Hanukkah menorah, representing the Jewish festival.",
            FAIcon::HardDrive => "A hard drive, representing computer storage.",
            FAIcon::Hashtag => "A hashtag symbol, representing social media or categorization.",
            FAIcon::HatCowboy => "A cowboy hat, representing Western style.",
            FAIcon::HatCowboySide => "A cowboy hat from the side, representing Western style.",
            FAIcon::HatWizard => "A wizard hat, representing magic or fantasy.",
            FAIcon::HeadSideCough => "A head coughing, representing illness or discomfort.",
            FAIcon::HeadSideCoughSlash => "A head with a slash, indicating no coughing.",
            FAIcon::HeadSideMask => "A head with a mask, representing health protection.",
            FAIcon::HeadSideVirus => "A head with a virus, representing infection or illness.",
            FAIcon::Heading => "A large capital letter 'A', representing text heading.",
            FAIcon::Headphones => "A pair of headphones, indicating audio or music listening.",
            FAIcon::HeadphonesSimple => "Simple headphones, representing audio listening.",
            FAIcon::Headset => "A headset, representing audio communication.",
            FAIcon::Heart => "A heart shape, symbolizing love or likes.",
            FAIcon::HeartCircleBolt => "A heart with a circled bolt, representing energetic love.",
            FAIcon::HeartCircleCheck => {
                "A heart with a circled check mark, representing acceptance or love."
            }
            FAIcon::HeartCircleExclamation => {
                "A heart inside a circle with an exclamation mark, representing urgent health."
            }
            FAIcon::HeartCircleMinus => {
                "A heart inside a circle with a minus sign, representing health reduction."
            }
            FAIcon::HeartCirclePlus => {
                "A heart inside a circle with a plus sign, representing health or medical support."
            }
            FAIcon::HeartCircleXmark => "A heart with a circled `X`, representing rejection or loss.",
            FAIcon::HeartCrack => "A broken heart, representing heartbreak or sadness.",
            FAIcon::HeartPulse => "A heart with a pulse line, representing health or cardiology.",
            FAIcon::Helicopter => "A helicopter, representing aviation.",
            FAIcon::HelicopterSymbol => "A helicopter in flight, representing aviation.",
            FAIcon::HelmetSafety => "A safety helmet, representing protection or construction.",
            FAIcon::HelmetUn => "A helmet with 'UN', representing United Nations peacekeepers.",
            FAIcon::Highlighter => "A highlighter pen, representing marking or emphasis.",
            FAIcon::HillAvalanche => "A hill with an avalanche, representing snow slides.",
            FAIcon::HillRockslide => "A hill with a rockslide, representing landslides.",
            FAIcon::Hippo => "An icon of a hippo, representing the animal.",
            FAIcon::HockeyPuck => "A hockey puck, representing the sport of hockey.",
            FAIcon::HollyBerry => "A holly berry, representing Christmas or winter.",
            FAIcon::Horse => "A horse, representing the animal.",
            FAIcon::HorseHead => "A horse head, representing the animal or chess piece.",
            FAIcon::Hospital => "A hospital building, representing healthcare services.",
            FAIcon::HospitalUser => "A hospital with a user, representing a healthcare facility.",
            FAIcon::HotTubPerson => "A person in a hot tub, representing relaxation or spa.",
            FAIcon::Hotdog => "A hotdog, representing fast food.",
            FAIcon::Hotel => "A bed with a person, representing a hotel or accommodation.",
            FAIcon::Hourglass => "An hourglass, representing time.",
            FAIcon::HourglassEnd => "An hourglass almost empty, representing time running out.",
            FAIcon::HourglassHalf => "An hourglass half full, representing time.",
            FAIcon::HourglassStart => {
                "An hourglass with sand at the top, indicating the start of a timer."
            }
            FAIcon::House => "A simple outline of a house.",
            FAIcon::HouseChimney => "A house with a chimney, representing a home.",
            FAIcon::HouseChimneyCrack => "A house with a chimney and crack, representing damage.",
            FAIcon::HouseChimneyMedical => {
                "A house with a chimney and medical symbol, representing a home medical facility."
            }
            FAIcon::HouseChimneyUser => {
                "A house with a chimney and user, representing a home resident."
            }
            FAIcon::HouseChimneyWindow => "A house with a chimney and window, representing a home.",
            FAIcon::HouseCircleCheck => "A house with a circled check mark, representing approval.",
            FAIcon::HouseCircleExclamation => {
                "A house with a circled exclamation mark, representing caution."
            }
            FAIcon::HouseCircleXmark => "A house with a circled `X`, representing exclusion.",
            FAIcon::HouseCrack => "A house with a crack, representing damage or earthquake.",
            FAIcon::HouseFire => "A house with flames, representing a fire emergency.",
            FAIcon::HouseFlag => "A house with a flag, representing home pride.",
            FAIcon::HouseFloodWater => "A house with water, representing flooding.",
            FAIcon::HouseFloodWaterCircleArrowRight => {
                "A house with water and an arrow, representing flood direction."
            }
            FAIcon::HouseLaptop => "A house with a laptop, representing remote work or home office.",
            FAIcon::HouseLock => "A house with a lock, representing home security.",
            FAIcon::HouseMedical => "A house with a medical symbol, representing a medical facility.",
            FAIcon::HouseMedicalCircleCheck => {
                "A house with a medical symbol and a check mark, representing an approved medical facility."
            }
            FAIcon::HouseMedicalCircleExclamation => {
                "A house with a medical symbol and an exclamation mark, representing a medical facility with caution."
            }
            FAIcon::HouseMedicalCircleXmark => {
                "A house with a medical symbol and an `X`, representing a medical facility with restriction."
            }
            FAIcon::HouseMedicalFlag => {
                "A house with a medical flag, representing a medical facility."
            }
            FAIcon::HouseSignal => "A house with a signal, representing smart home or connectivity.",
            FAIcon::HouseTsunami => "A house with a tsunami wave, representing natural disaster.",
            FAIcon::HouseUser => "A house with a user inside, representing home or resident.",
            FAIcon::HryvniaSign => "The symbol for the Ukrainian hryvnia, indicating currency.",
            FAIcon::Hurricane => "A hurricane symbol, representing severe weather.",
            FAIcon::I => "The letter \"I\", representing the alphabet.",
            FAIcon::ICursor => "An I-beam cursor, representing text selection.",
            FAIcon::IceCream => "An ice cream cone, representing dessert or treats.",
            FAIcon::Icicles => "Ice formations, representing cold or winter.",
            FAIcon::Icons => "A collection of small icons, representing various symbols.",
            FAIcon::IdBadge => "An ID badge, representing identification.",
            FAIcon::IdCard => "An ID card, representing identification.",
            FAIcon::IdCardClip => "An ID card with a clip, representing identification.",
            FAIcon::Igloo => "An igloo, representing an Inuit house or cold regions.",
            FAIcon::Image => "A picture or photo icon, representing image content.",
            FAIcon::ImagePortrait => "A portrait image, representing photos or profiles.",
            FAIcon::Images => "Multiple images, representing photo galleries or collections.",
            FAIcon::Inbox => {
                "A tray filled with documents, representing an inbox or received messages."
            }
            FAIcon::Indent => "An indented line, representing text formatting.",
            FAIcon::IndianRupeeSign => "The symbol for the Indian rupee, indicating currency.",
            FAIcon::Industry => "A factory, representing industry or manufacturing.",
            FAIcon::Infinity => "An infinity symbol, representing limitless or infinite.",
            FAIcon::Info => "A lowercase 'i' in a circle, indicating information.",
            FAIcon::Instagram => "The logo of Instagram, representing the social media platform.",
            FAIcon::Italic => "An italic 'I', representing italicized text.",
            FAIcon::J => "A capital letter 'J', representing the letter.",
            FAIcon::Jar => "A simple jar, representing containers or storage.",
            FAIcon::JarWheat => "A jar with wheat, representing food storage.",
            FAIcon::Jedi => "The symbol for the Jedi Order, representing Star Wars or spirituality.",
            FAIcon::JediOrder => "The logo of Jedi Order, representing the Star Wars faction.",
            FAIcon::JetFighter => "A jet fighter, representing aviation or military.",
            FAIcon::JetFighterUp => "A jet fighter pointing up, representing aviation or military.",
            FAIcon::Joint => "Two linked circles, representing a connection or joint.",
            FAIcon::JugDetergent => "A jug of detergent, representing cleaning supplies.",
            FAIcon::K => "A capital letter 'K', representing the letter.",
            FAIcon::Kaaba => "The Kaaba, representing the holy site in Islam.",
            FAIcon::Key => "A key, representing access or security.",
            FAIcon::Keyboard => "A keyboard, representing typing or computing.",
            FAIcon::Khanda => "The symbol for Khanda, representing Sikhism.",
            FAIcon::Kickstarter => "The logo of Kickstarter, a crowdfunding platform.",
            FAIcon::KipSign => "The symbol for the Lao kip, indicating currency.",
            FAIcon::KitMedical => "A medical kit, representing health supplies.",
            FAIcon::KitchenSet => "Kitchen utensils, representing cooking or kitchen.",
            FAIcon::KiwiBird => "A kiwi bird, representing the bird or New Zealand.",
            FAIcon::L => "A capital letter 'L', representing the letter.",
            FAIcon::LandMineOn => "A land mine, representing explosive devices.",
            FAIcon::Landmark => "A landmark, representing important places or structures.",
            FAIcon::LandmarkDome => "A landmark with a dome, representing a notable building.",
            FAIcon::LandmarkFlag => "A landmark with a flag, representing a notable location.",
            FAIcon::Language => "A globe with characters, indicating language or translation.",
            FAIcon::Laptop => "A laptop computer, representing computing or work.",
            FAIcon::LaptopCode => "A laptop with code, representing programming or development.",
            FAIcon::LaptopFile => "A laptop with a file, representing digital documents.",
            FAIcon::LaptopMedical => {
                "A laptop with a medical symbol, representing telehealth or medical records."
            }
            FAIcon::LariSign => "The symbol for the Georgian lari, indicating currency.",
            FAIcon::LayerGroup => "Three stacked layers, indicating layering or grouping.",
            FAIcon::Leaf => "A leaf, representing nature or eco-friendliness.",
            FAIcon::LeftLong => "A long arrow pointing left, indicating direction.",
            FAIcon::LeftRight => {
                "An arrow pointing left and right, indicating bidirectional movement."
            }
            FAIcon::Lemon => "A lemon fruit, indicating the fruit or something sour.",
            FAIcon::LessThan => "A less than sign, representing mathematical operations.",
            FAIcon::LessThanEqual => {
                "A less than or equal sign, representing mathematical operations."
            }
            FAIcon::LifeRing => "A life ring, representing safety or rescue.",
            FAIcon::Lightbulb => "A lightbulb, representing ideas or illumination.",
            FAIcon::LinesLeaning => "Leaning lines, representing design or structure.",
            FAIcon::Link => "A chain link, indicating a hyperlink or connection.",
            FAIcon::LinkSlash => "A broken link, representing a disconnected hyperlink.",
            FAIcon::Linkedin => {
                "The logo of LinkedIn, representing the professional networking site."
            }
            FAIcon::LiraSign => "The symbol for the Italian lira, indicating currency.",
            FAIcon::List => "A simple list, representing items or data.",
            FAIcon::ListCheck => "A list with check marks, representing tasks or to-do lists.",
            FAIcon::ListOl => "An ordered list, representing a sequence or ranking.",
            FAIcon::ListUl => "A list with bullet points, representing unordered lists.",
            FAIcon::LitecoinSign => "The symbol for Litecoin, indicating cryptocurrency.",
            FAIcon::LocationArrow => "An arrow on a map, representing direction.",
            FAIcon::LocationCrosshairs => "A crosshairs on a map, representing target location.",
            FAIcon::LocationDot => "A pinpoint marker, indicating a location on a map.",
            FAIcon::LocationPin => "A pin marker, indicating a specific location.",
            FAIcon::LocationPinLock => "A location pin with a lock, representing secure location.",
            FAIcon::Lock => "A padlock, representing security or privacy.",
            FAIcon::LockOpen => "A padlock that is open, representing access or security.",
            FAIcon::Locust => "A locust, representing the insect or a plague.",
            FAIcon::Lungs => "A pair of lungs, representing the respiratory system.",
            FAIcon::LungsVirus => "A pair of lungs with a virus, representing respiratory illness.",
            FAIcon::M => "A capital letter 'M', representing the letter.",
            FAIcon::Magnet => "A horseshoe magnet, representing attraction or magnetic fields.",
            FAIcon::MagnifyingGlass => {
                "A magnifying glass, often used to represent search functionality."
            }
            FAIcon::MagnifyingGlassArrowRight => {
                "A magnifying glass with a right arrow, representing search direction."
            }
            FAIcon::MagnifyingGlassChart => {
                "A magnifying glass with a chart, representing detailed analysis."
            }
            FAIcon::MagnifyingGlassDollar => {
                "A magnifying glass with a dollar sign, representing financial search."
            }
            FAIcon::MagnifyingGlassLocation => {
                "A magnifying glass over a location pin, representing search location."
            }
            FAIcon::MagnifyingGlassMinus => {
                "A magnifying glass with a minus sign, representing zoom out or search."
            }
            FAIcon::MagnifyingGlassPlus => {
                "A magnifying glass with a plus sign, representing zoom in or search."
            }
            FAIcon::ManatSign => "The symbol for the Azerbaijani manat, indicating currency.",
            FAIcon::Map => "A folded map, representing navigation or geography.",
            FAIcon::MapLocation => "A map with a pin, representing location or navigation.",
            FAIcon::MapLocationDot => "A map pin with a dot, representing location or navigation.",
            FAIcon::MapPin => "A map pin, representing location or navigation.",
            FAIcon::Marker => "A marker, representing writing or drawing tools.",
            FAIcon::Mars => "The symbol for Mars, representing the planet or male gender.",
            FAIcon::MarsAndVenus => {
                "The symbols for Mars and Venus, representing gender or relationships."
            }
            FAIcon::MarsAndVenusBurst => {
                "The symbols for Mars and Venus with a burst, indicating gender diversity."
            }
            FAIcon::MarsDouble => "Two Mars symbols, representing male gender or masculinity.",
            FAIcon::MarsStroke => {
                "The Mars stroke symbol, representing a variation of the male gender symbol."
            }
            FAIcon::MarsStrokeRight => {
                "The symbol for Mars with a right arrow, indicating male gender or masculinity."
            }
            FAIcon::MarsStrokeUp => {
                "The symbol for Mars with an upward arrow, indicating male gender or masculinity."
            }
            FAIcon::MartiniGlass => "A martini glass, representing beverages or cocktails.",
            FAIcon::MartiniGlassCitrus => {
                "A martini glass with a citrus slice, representing beverages or cocktails."
            }
            FAIcon::MartiniGlassEmpty => {
                "An empty martini glass, representing beverages or cocktails."
            }
            FAIcon::Mask => "A theater mask, representing performance or disguise.",
            FAIcon::MaskFace => "A face mask, representing health or safety.",
            FAIcon::MaskVentilator => "A medical mask, representing health protection.",
            FAIcon::MasksTheater => "Two theater masks, representing performance or drama.",
            FAIcon::MattressPillow => "A mattress with a pillow, representing bedding or sleep.",
            FAIcon::Maximize => "A square with arrows pointing outwards, indicating maximization.",
            FAIcon::Medal => "A medal, representing achievement or award.",
            FAIcon::Medium => "The logo of Medium, a publishing platform.",
            FAIcon::Memory => "A microchip, representing memory or computing hardware.",
            FAIcon::Menorah => "A menorah, representing the Jewish candelabrum.",
            FAIcon::Mercury => {
                "The symbol for the planet Mercury, representing the celestial body or the element."
            }
            FAIcon::Message => "A speech bubble, representing communication or messaging.",
            FAIcon::Meteor => "A meteor, representing space or celestial events.",
            FAIcon::Microchip => "A microchip, representing technology or computing.",
            FAIcon::Microphone => "A microphone, representing audio or recording.",
            FAIcon::MicrophoneLines => {
                "A microphone with lines, representing audio recording or broadcasting."
            }
            FAIcon::MicrophoneLinesSlash => {
                "A microphone with a slash, indicating no audio recording."
            }
            FAIcon::MicrophoneSlash => "A microphone with a slash, indicating mute or no sound.",
            FAIcon::Microscope => "A microscope, representing science or research.",
            FAIcon::MillSign => "A sign for mills, representing currency or measurement.",
            FAIcon::Minimize => "A minimized window, representing reduction.",
            FAIcon::Minus => "A minus sign, indicating subtraction or decrease.",
            FAIcon::Mitten => "A mitten, representing winter clothing.",
            FAIcon::Mobile => "A mobile phone, indicating communication or devices.",
            FAIcon::MobileButton => {
                "A mobile phone with buttons, representing old-style mobile device."
            }
            FAIcon::MobileRetro => "A retro mobile phone, representing old technology.",
            FAIcon::MobileScreen => "A mobile phone, representing mobile device.",
            FAIcon::MobileScreenButton => "A mobile phone with a button, representing mobile device.",
            FAIcon::MoneyBill => "A paper bill, representing money or currency.",
            FAIcon::MoneyBill1 => "A money bill, representing payment or currency.",
            FAIcon::MoneyBill1Wave => {
                "A money bill with a wave, representing payment or transaction."
            }
            FAIcon::MoneyBillTransfer => "A money bill with an arrow, indicating financial transfer.",
            FAIcon::MoneyBillTrendUp => {
                "A money bill with an upward trend, representing financial growth."
            }
            FAIcon::MoneyBillWave => "A waving money bill, representing cash flow.",
            FAIcon::MoneyBillWheat => {
                "A money bill with wheat, representing agricultural subsidy or trade."
            }
            FAIcon::MoneyBills => "A stack of money bills, representing wealth or currency.",
            FAIcon::MoneyCheck => "A check, representing financial transactions.",
            FAIcon::MoneyCheckDollar => {
                "A check with a dollar sign, representing financial transactions."
            }
            FAIcon::Monument => "A monument, representing historical or cultural significance.",
            FAIcon::Moon => "A crescent moon, representing night or sleep mode.",
            FAIcon::MortarPestle => "A mortar and pestle, representing grinding or pharmacy.",
            FAIcon::Mosque => "A mosque, representing Islamic place of worship.",
            FAIcon::Mosquito => "A mosquito, representing the insect or disease vector.",
            FAIcon::MosquitoNet => "A mosquito net, representing protection from insects.",
            FAIcon::Motorcycle => "A motorcycle, representing motorbiking.",
            FAIcon::Mound => "A mound of earth, representing a small hill or pile.",
            FAIcon::Mountain => "A mountain, representing nature or hiking.",
            FAIcon::MountainCity => {
                "A cityscape with mountains, representing urban and natural landscape."
            }
            FAIcon::MountainSun => "A mountain with a sun, indicating landscape or outdoors.",
            FAIcon::MugHot => "A hot mug, representing a hot beverage.",
            FAIcon::MugSaucer => "A mug on a saucer, representing coffee or tea.",
            FAIcon::Music => "A musical note, representing music or audio.",
            FAIcon::N => "A capital letter 'N', representing the letter.",
            FAIcon::NairaSign => "The symbol for the Nigerian naira, indicating currency.",
            FAIcon::Napster => "The logo of Napster, representing the music streaming service.",
            FAIcon::NetworkWired => "A network of connected nodes, representing wired networking.",
            FAIcon::Neuter => "The gender symbol for neuter, indicating neutrality.",
            FAIcon::Newspaper => "A newspaper, indicating news or publications.",
            FAIcon::NfcDirectional => {
                "The logo of NFC Directional, representing near-field communication."
            }
            FAIcon::NfcSymbol => {
                "The NFC (Near Field Communication) symbol, representing wireless communication."
            }
            FAIcon::NotEqual => "A not equal sign, indicating inequality or difference.",
            FAIcon::Notdef => "The .notdef glyph, representing missing characters in typography.",
            FAIcon::NoteSticky => "A sticky note, representing reminders or notes.",
            FAIcon::NotesMedical => {
                "A clipboard with medical notes, representing healthcare documentation."
            }
            FAIcon::O => "A capital letter 'O', representing the letter or shape.",
            FAIcon::ObjectGroup => "An icon of grouped objects, indicating grouping.",
            FAIcon::ObjectUngroup => "An icon of separated objects, indicating ungrouping.",
            FAIcon::OilCan => "An oil can, representing lubrication or mechanics.",
            FAIcon::OilWell => "An oil well, representing fossil fuels or drilling.",
            FAIcon::OldRepublic => "The logo of Old Republic, representing the Star Wars faction.",
            FAIcon::Om => "The Om symbol, representing Hinduism.",
            FAIcon::Otter => "An otter, representing the animal.",
            FAIcon::Outdent => "Text with a reduced indent, representing text alignment.",
            FAIcon::P => "A capital letter 'P', representing the letter or parking.",
            FAIcon::Pager => "A pager, representing communication devices.",
            FAIcon::PaintRoller => "A paint roller, indicating painting or renovation.",
            FAIcon::Paintbrush => "A paintbrush, representing painting or art.",
            FAIcon::Palette => "A painter's palette, representing art or color selection.",
            FAIcon::Pallet => "A pallet, representing shipping or logistics.",
            FAIcon::Panorama => "A wide-angle view, representing landscape photography.",
            FAIcon::PaperPlane => "A paper plane, indicating sending a message or flying.",
            FAIcon::Paperclip => "A paperclip, representing attachment or link.",
            FAIcon::ParachuteBox => "A box with a parachute, representing delivery or drop.",
            FAIcon::Paragraph => "A paragraph symbol, representing text.",
            FAIcon::Passport => "A passport, representing international travel.",
            FAIcon::Paste => "A clipboard with a document, representing pasting.",
            FAIcon::Pause => "A pause symbol, representing media pause.",
            FAIcon::Paw => "A paw print, representing animals or pets.",
            FAIcon::Paypal => "The logo of PayPal, an online payment system.",
            FAIcon::Peace => "A peace symbol, representing peace or anti-war.",
            FAIcon::Pen => "A pen, representing writing or creativity.",
            FAIcon::PenClip => "A pen with a clip, representing writing or stationery.",
            FAIcon::PenFancy => "A fancy pen, representing writing or creativity.",
            FAIcon::PenNib => "An old-fashioned pen nib, representing writing or creativity.",
            FAIcon::PenRuler => "A pen and ruler, representing drawing or design.",
            FAIcon::PenToSquare => "A pen writing on a square, indicating editing or writing.",
            FAIcon::Pencil => "A pencil, representing writing or editing.",
            FAIcon::PeopleArrows => {
                "Two people with arrows pointing towards each other, representing communication or interaction."
            }
            FAIcon::PeopleCarryBox => "People carrying a box, representing moving or teamwork.",
            FAIcon::PeopleGroup => "Multiple people, representing a group or community.",
            FAIcon::PeopleLine => "People standing in line, representing queue.",
            FAIcon::PeoplePulling => "Two people pulling, representing teamwork or effort.",
            FAIcon::PeopleRobbery => "A person being robbed, representing crime or danger.",
            FAIcon::PeopleRoof => "People under a roof, representing shelter or protection.",
            FAIcon::PepperHot => "A hot pepper, representing spicy food.",
            FAIcon::Percent => "A percent sign, indicating percentages or discounts.",
            FAIcon::Person => "A person, representing an individual or user.",
            FAIcon::PersonArrowDownToLine => {
                "A person with an arrow pointing down to a line, indicating descending or moving down."
            }
            FAIcon::PersonArrowUpFromLine => {
                "A person with an arrow pointing up from a line, indicating rising or moving up."
            }
            FAIcon::PersonBiking => "A person biking, representing cycling.",
            FAIcon::PersonBooth => "A person in a booth, indicating privacy or voting.",
            FAIcon::PersonBreastfeeding => {
                "A person breastfeeding, representing motherhood or childcare."
            }
            FAIcon::PersonBurst => "A person with a burst, indicating excitement or energy.",
            FAIcon::PersonCane => "A person with a cane, representing disability or assistance.",
            FAIcon::PersonChalkboard => {
                "A person at a chalkboard, representing teaching or presentation."
            }
            FAIcon::PersonCircleCheck => {
                "A person inside a circle with a check mark, representing verification."
            }
            FAIcon::PersonCircleExclamation => {
                "A person with a circled exclamation mark, indicating warning."
            }
            FAIcon::PersonCircleMinus => {
                "A person with a circled minus, indicating removal or exclusion."
            }
            FAIcon::PersonCirclePlus => {
                "A person with a circled plus, indicating addition or inclusion."
            }
            FAIcon::PersonCircleQuestion => {
                "A person with a circled question mark, indicating inquiry or uncertainty."
            }
            FAIcon::PersonCircleXmark => "A person with a circled `X`, indicating exclusion.",
            FAIcon::PersonDigging => "A person digging, representing construction or excavation.",
            FAIcon::PersonDotsFromLine => {
                "A person with dots moving from a line, representing transition or movement."
            }
            FAIcon::PersonDress => "A person wearing a dress, representing clothing or fashion.",
            FAIcon::PersonDressBurst => {
                "A person in a dress with a burst, indicating excitement or motion."
            }
            FAIcon::PersonDrowning => "A person drowning, representing danger in water.",
            FAIcon::PersonFalling => "A person falling, representing accident or failure.",
            FAIcon::PersonFallingBurst => {
                "A person falling with a burst, representing injury or accident."
            }
            FAIcon::PersonHalfDress => {
                "A person wearing half a dress, representing fashion or gender fluidity."
            }
            FAIcon::PersonHarassing => "A person harassing another, representing harassment.",
            FAIcon::PersonHiking => "A person hiking, representing outdoor activities.",
            FAIcon::PersonMilitaryPointing => {
                "A military person pointing, indicating direction or command."
            }
            FAIcon::PersonMilitaryRifle => {
                "A military person holding a rifle, representing armed forces."
            }
            FAIcon::PersonMilitaryToPerson => {
                "A military person saluting another person, representing respect."
            }
            FAIcon::PersonPraying => "A person praying, representing spirituality or religion.",
            FAIcon::PersonPregnant => "A pregnant person, representing pregnancy.",
            FAIcon::PersonRays => "A person with rays, representing radiance or positivity.",
            FAIcon::PersonRifle => {
                "A person holding a rifle, representing shooting sports or military."
            }
            FAIcon::PersonRunning => "A person running, representing movement or exercise.",
            FAIcon::PersonShelter => "A person under a shelter, representing protection or safety.",
            FAIcon::PersonSkating => "A person skating, representing the sport or activity.",
            FAIcon::PersonSkiing => "A person skiing, representing winter sports.",
            FAIcon::PersonSkiingNordic => "A person skiing Nordic style, representing skiing.",
            FAIcon::PersonSnowboarding => "A person snowboarding, representing winter sports.",
            FAIcon::PersonSwimming => "A person swimming, representing swimming or water sports.",
            FAIcon::PersonThroughWindow => {
                "A person moving through a window, indicating escape or emergency exit."
            }
            FAIcon::PersonWalking => "A person walking, representing movement.",
            FAIcon::PersonWalkingArrowLoopLeft => {
                "A person walking with a looping arrow to the left, indicating return or reverse."
            }
            FAIcon::PersonWalkingArrowRight => "A person walking with an arrow, indicating movement.",
            FAIcon::PersonWalkingDashedLineArrowRight => {
                "A person walking with a dashed line and arrow, indicating a guided path."
            }
            FAIcon::PersonWalkingLuggage => "A person walking with luggage, indicating travel.",
            FAIcon::PersonWalkingWithCane => {
                "A person walking with a cane, indicating disability or assistance."
            }
            FAIcon::PesetaSign => "The symbol for the Spanish peseta, indicating currency.",
            FAIcon::PesoSign => "The symbol for the Philippine peso, indicating currency.",
            FAIcon::Phone => "A phone, representing communication or contact.",
            FAIcon::PhoneFlip => "A phone flipped, indicating mobile communication.",
            FAIcon::PhoneSlash => "A phone with a slash, indicating no calls.",
            FAIcon::PhoneVolume => {
                "A phone handset with sound waves, indicating a call or audio settings."
            }
            FAIcon::PhotoFilm => "A strip of photo film, representing photography.",
            FAIcon::PiggyBank => "A piggy bank, representing savings or finance.",
            FAIcon::Pills => "A pair of pills, representing medication.",
            FAIcon::PizzaSlice => "A slice of pizza, representing food or dining.",
            FAIcon::PlaceOfWorship => "A place of worship, indicating religious services.",
            FAIcon::Plane => "An airplane, indicating travel or flights.",
            FAIcon::PlaneArrival => "A plane arriving, indicating air travel arrival.",
            FAIcon::PlaneCircleCheck => {
                "A plane with a circled check mark, representing flight confirmation."
            }
            FAIcon::PlaneCircleExclamation => {
                "A plane with a circled exclamation mark, indicating travel alert."
            }
            FAIcon::PlaneCircleXmark => "A plane with a circled `X`, indicating no flying.",
            FAIcon::PlaneDeparture => "A plane taking off, indicating air travel.",
            FAIcon::PlaneLock => "A plane with a lock, indicating secure travel.",
            FAIcon::PlaneSlash => "A plane with a slash, indicating no flying.",
            FAIcon::PlaneUp => "A plane with the tip upwards.",
            FAIcon::PlantWilt => "A wilted plant, indicating lack of water or poor health.",
            FAIcon::PlateWheat => "A plate with wheat, indicating food or meal.",
            FAIcon::Play => "A play button, indicating media playback.",
            FAIcon::Playstation => "The logo of PlayStation, a gaming console.",
            FAIcon::Plug => "An electrical plug, indicating power or connectivity.",
            FAIcon::PlugCircleBolt => "A plug with a circled bolt, indicating powered connection.",
            FAIcon::PlugCircleCheck => "A plug with a circled check, indicating secure connection.",
            FAIcon::PlugCircleExclamation => {
                "A plug with a circled exclamation mark, representing power alert."
            }
            FAIcon::PlugCircleMinus => {
                "A plug with a circled minus sign, representing power reduction."
            }
            FAIcon::PlugCirclePlus => "A plug with a circled plus, indicating connection.",
            FAIcon::PlugCircleXmark => "A plug with a circled `X`, indicating no connection.",
            FAIcon::Plus => "A cross, representing addition or positivity.",
            FAIcon::PlusMinus => "A plus and minus sign, indicating addition and subtraction.",
            FAIcon::Podcast => "A podcast icon, representing audio broadcasting.",
            FAIcon::Poo => "A pile of poo with eyes, often used humorously.",
            FAIcon::PooStorm => "A storm cloud with a poo, often used humorously.",
            FAIcon::Poop => "A pile of poop, representing waste or humor.",
            FAIcon::PowerOff => "A power button, indicating shutdown or turning off.",
            FAIcon::Prescription => "A prescription symbol, indicating medical prescription.",
            FAIcon::PrescriptionBottle => {
                "A prescription bottle, representing medicine or healthcare."
            }
            FAIcon::PrescriptionBottleMedical => {
                "A medical prescription bottle, indicating medication."
            }
            FAIcon::Print => "A printer, representing printing documents.",
            FAIcon::PumpMedical => "A medical pump, indicating medical equipment.",
            FAIcon::PumpSoap => "A soap dispenser, representing hygiene or cleanliness.",
            FAIcon::PuzzlePiece => "A puzzle piece, indicating a part of a puzzle.",
            FAIcon::Q => "The letter \"Q\", representing the alphabet.",
            FAIcon::Qrcode => "A QR code, representing quick response codes for scanning.",
            FAIcon::Question => "A question mark, indicating inquiry or help.",
            FAIcon::QuoteLeft => "A left-leaning quotation mark, indicating the start of a quote.",
            FAIcon::QuoteRight => "A right-leaning quotation mark, indicating the end of a quote.",
            FAIcon::R => "A capital letter 'R', representing the letter or registered trademark.",
            FAIcon::Radiation => "A radiation symbol, indicating hazardous materials.",
            FAIcon::Radio => "A radio, representing broadcasting or communication.",
            FAIcon::Rainbow => "A rainbow, representing LGBTQ+ pride or spectrum.",
            FAIcon::RankingStar => "A star with a number, indicating rank or rating.",
            FAIcon::Receipt => "A receipt, representing a transaction record.",
            FAIcon::RecordVinyl => "A vinyl record, representing music or audio.",
            FAIcon::RectangleAd => "A rectangle with 'AD' inside, indicating advertisement.",
            FAIcon::RectangleList => "A rectangle with a list inside, representing menu or options.",
            FAIcon::RectangleXmark => "A rectangle with an `X`, indicating deletion or closure.",
            FAIcon::Recycle => "Three arrows forming a triangle, indicating recycling.",
            FAIcon::Registered => "A circled 'R', indicating a registered trademark.",
            FAIcon::Repeat => "Two arrows forming a circle, indicating repeat or refresh.",
            FAIcon::Reply => "An arrow pointing left, indicating a reply.",
            FAIcon::ReplyAll => "A reply-all symbol, representing email or messaging.",
            FAIcon::Republican => "An elephant, representing the Republican party.",
            FAIcon::Restroom => "A man and woman icon, indicating restroom facilities.",
            FAIcon::Retweet => "Two arrows forming a square, indicating retweet or repost.",
            FAIcon::Ribbon => "A ribbon, representing awareness or decoration.",
            FAIcon::RightFromBracket => {
                "An arrow pointing right from a bracket, indicating exit or move."
            }
            FAIcon::RightLeft => {
                "An arrow pointing right and left, indicating bidirectional movement."
            }
            FAIcon::RightLong => "A long arrow pointing right, indicating forward direction.",
            FAIcon::RightToBracket => {
                "An arrow pointing right into a bracket, indicating entering or logging in."
            }
            FAIcon::Ring => "A ring, representing jewelry or engagement.",
            FAIcon::Road => "A road, indicating travel or transportation.",
            FAIcon::RoadBarrier => "A road barrier, indicating roadblock or construction.",
            FAIcon::RoadBridge => "A bridge, representing transportation infrastructure.",
            FAIcon::RoadCircleCheck => {
                "A road with a circled check mark, representing approved routes."
            }
            FAIcon::RoadCircleExclamation => {
                "A road with a circled exclamation mark, indicating caution or warning."
            }
            FAIcon::RoadCircleXmark => "A road with a circled `X`, indicating road closure.",
            FAIcon::RoadLock => "A road with a lock, indicating restricted access.",
            FAIcon::RoadSpikes => "Spikes on the road, representing security or vehicle stop.",
            FAIcon::Robot => "A robot, representing automation or robotics.",
            FAIcon::Rocket => "A rocket, indicating space exploration or rapid progress.",
            FAIcon::Rotate => "A circular arrow, indicating rotation or refresh.",
            FAIcon::RotateLeft => "An arrow rotating to the left, indicating undo or backward.",
            FAIcon::RotateRight => "An arrow rotating to the right, indicating redo or refresh.",
            FAIcon::Route => "A winding road, indicating a path or journey.",
            FAIcon::Rss => "A feed icon, representing RSS feed.",
            FAIcon::RubleSign => "The symbol for the Russian ruble, indicating currency.",
            FAIcon::Rug => "A rug, representing home decor or carpeting.",
            FAIcon::Ruler => "A ruler, representing measurement.",
            FAIcon::RulerCombined => "A ruler combined with another tool, representing measurement.",
            FAIcon::RulerHorizontal => "A horizontal ruler, representing measurement.",
            FAIcon::RulerVertical => "A vertical ruler, representing measurement.",
            FAIcon::RupeeSign => "The symbol for the Indian rupee, indicating currency in letters",
            FAIcon::RupiahSign => "The symbol for the Indonesian rupiah, indicating currency.",
            FAIcon::S => "A capital letter 'S', representing the letter or Superman.",
            FAIcon::SackDollar => "A sack with a dollar sign, indicating money or wealth.",
            FAIcon::SackXmark => "A sack with an `X`, indicating no contents or emptiness.",
            FAIcon::Sailboat => "A sailboat, representing sailing or maritime activities.",
            FAIcon::Satellite => "A satellite, representing space or communication.",
            FAIcon::SatelliteDish => "A satellite dish, representing communication.",
            FAIcon::ScaleBalanced => "A balanced scale, representing justice or equality.",
            FAIcon::ScaleUnbalanced => "A tilted scale, indicating imbalance.",
            FAIcon::ScaleUnbalancedFlip => {
                "A tilted scale, indicating imbalance, flipped horizzontally."
            }
            FAIcon::School => "A school building, representing education.",
            FAIcon::SchoolCircleCheck => {
                "A school building with a check mark in a circle, indicating school approval."
            }
            FAIcon::SchoolCircleExclamation => {
                "A school with a circled exclamation mark, representing school alert."
            }
            FAIcon::SchoolCircleXmark => {
                "A school with a circled X, representing school closure or cancellation."
            }
            FAIcon::SchoolFlag => {
                "A school building with a flag, representing education or school pride."
            }
            FAIcon::SchoolLock => "A school building with a lock, indicating school security.",
            FAIcon::Scissors => "A pair of scissors, representing cutting or crafting.",
            FAIcon::Screwdriver => "A screwdriver, representing tools or repair.",
            FAIcon::ScrewdriverWrench => {
                "A screwdriver and wrench crossed, representing tools or repair."
            }
            FAIcon::Scroll => "A scroll, representing a document or parchment.",
            FAIcon::ScrollTorah => "A scroll, representing the Torah or ancient texts.",
            FAIcon::SdCard => "An SD card, representing storage or memory.",
            FAIcon::Section => "A divided section, representing a part or segment.",
            FAIcon::Seedling => "A small plant sprouting, representing growth or new beginnings.",
            FAIcon::Server => "A server, representing data storage or hosting.",
            FAIcon::Shapes => "A collection of geometric shapes, representing design or layout.",
            FAIcon::Share => "An arrow pointing outwards, indicating sharing content.",
            FAIcon::ShareFromSquare => {
                "An arrow coming out of a square, indicating sharing or exporting."
            }
            FAIcon::ShareNodes => "Three connected nodes, representing sharing or networking.",
            FAIcon::SheetPlastic => "A sheet of plastic, representing material.",
            FAIcon::ShekelSign => "The symbol for the Israeli shekel, indicating currency.",
            FAIcon::Shield => "A shield, representing protection or security.",
            FAIcon::ShieldCat => "A shield with a cat, representing pet protection.",
            FAIcon::ShieldDog => "A shield with a dog, representing pet protection.",
            FAIcon::ShieldHalved => {
                "A shield split in half, indicating partial protection or security."
            }
            FAIcon::ShieldHeart => "A shield with a heart, representing health protection.",
            FAIcon::ShieldVirus => "A shield with a virus, representing antivirus protection.",
            FAIcon::Ship => "A ship, representing maritime transportation.",
            FAIcon::Shirt => "A t-shirt, indicating clothing.",
            FAIcon::ShoePrints => "Shoe prints, representing footsteps or tracking.",
            FAIcon::Shop => "A store front, indicating shopping or retail.",
            FAIcon::ShopLock => "A shop with a lock, representing a closed store.",
            FAIcon::ShopSlash => "A shop with a slash, indicating closed or no shopping.",
            FAIcon::Shopify => "The logo of Shopify, an e-commerce platform.",
            FAIcon::Shower => "A shower head with water, indicating bathing.",
            FAIcon::Shrimp => "A shrimp, representing seafood.",
            FAIcon::Shuffle => "Two arrows crossing, indicating shuffle or random order.",
            FAIcon::ShuttleSpace => "A space shuttle, representing space exploration.",
            FAIcon::SignHanging => "A hanging sign, representing a signboard or notice.",
            FAIcon::Signal => {
                "A signal tower with waves, representing communication or connectivity."
            }
            FAIcon::Signature => "A handwritten signature, indicating signing or approval.",
            FAIcon::SignsPost => "A signpost, representing directions or navigation.",
            FAIcon::SimCard => "A SIM card, representing mobile connectivity.",
            FAIcon::Sink => "A sink, representing kitchen or bathroom fixtures.",
            FAIcon::Sitemap => {
                "A hierarchical diagram, representing a sitemap or organization chart."
            }
            FAIcon::Skull => "A simple skull, representing death or danger.",
            FAIcon::SkullCrossbones => "A skull with crossbones, representing danger or pirates.",
            FAIcon::Slack => "The logo of Slack, a communication platform for teams.",
            FAIcon::Slash => "A slash symbol, representing separation or division.",
            FAIcon::Sleigh => "A sleigh, representing Christmas or winter transport.",
            FAIcon::Sliders => "Sliders, representing controls or adjustments.",
            FAIcon::Smog => "A city skyline with smog, representing air pollution.",
            FAIcon::Smoking => "A cigarette with smoke, representing smoking.",
            FAIcon::Snowflake => "A snowflake, representing cold or winter.",
            FAIcon::Snowman => "A snowman, representing winter or Christmas.",
            FAIcon::Snowplow => "A snowplow vehicle, representing snow removal.",
            FAIcon::Soap => "A bar of soap, representing cleanliness or hygiene.",
            FAIcon::Socks => "A pair of socks, representing clothing.",
            FAIcon::SolarPanel => "A solar panel, representing solar energy.",
            FAIcon::Sort => "Three stacked horizontal lines, indicating sorting.",
            FAIcon::SortDown => {
                "A list with a downward arrow, indicating sorting in descending order."
            }
            FAIcon::SortUp => "A list with an upward arrow, indicating sorting in ascending order.",
            FAIcon::Soundcloud => "The logo of SoundCloud, representing the music platform.",
            FAIcon::Spa => "A flower with petals, representing relaxation or spa.",
            FAIcon::SpaceAwesome => "The logo of Space Awesome, representing the brand or company.",
            FAIcon::SpaghettiMonsterFlying => {
                "A flying spaghetti monster, representing parody religion."
            }
            FAIcon::SpellCheck => "A check mark with ABC, representing spell checking.",
            FAIcon::Spider => "A spider, representing the arachnid or Halloween.",
            FAIcon::Spinner => "A spinning circle, indicating loading or processing.",
            FAIcon::Splotch => "A paint splotch, representing color or mess.",
            FAIcon::Spoon => "A spoon, representing dining or kitchen utensils.",
            FAIcon::Spotify => "The logo of Spotify, a music streaming service.",
            FAIcon::SprayCan => "A spray can, representing painting or spraying.",
            FAIcon::SprayCanSparkles => "A spray can emitting sparkles, representing spray effects.",
            FAIcon::Square => "A simple square, representing shape or stop.",
            FAIcon::SquareArrowUpRight => {
                "A square with an arrow pointing up and right, indicating expansion or exit."
            }
            FAIcon::SquareCaretDown => {
                "A square with a downward caret, representing more options or dropdowns."
            }
            FAIcon::SquareCaretLeft => {
                "A square with a leftward caret, representing navigation or more options."
            }
            FAIcon::SquareCaretRight => {
                "A square with a rightward caret, representing navigation or more options."
            }
            FAIcon::SquareCaretUp => {
                "A square with an upward caret, representing navigation or more options."
            }
            FAIcon::SquareCheck => "A square with a check mark, indicating completion or approval.",
            FAIcon::SquareEnvelope => "A square with an envelope, representing mail or messages.",
            FAIcon::SquareFull => {
                "A square completely filled, representing fullness or completeness."
            }
            FAIcon::SquareH => "A square with an 'H', representing hospital.",
            FAIcon::SquareMinus => "A square with a minus sign, indicating removal or decrease.",
            FAIcon::SquareNfi => "A square with 'NFI', indicating an undefined acronym.",
            FAIcon::SquareParking => "A square with a 'P', representing parking.",
            FAIcon::SquarePen => "A square with a pen, representing editing or writing.",
            FAIcon::SquarePersonConfined => {
                "A square with a person confined inside, representing isolation."
            }
            FAIcon::SquarePhone => {
                "A square with a phone icon, representing communication or device."
            }
            FAIcon::SquarePhoneFlip => {
                "A square with a phone icon flipped, indicating phone rotation."
            }
            FAIcon::SquarePlus => "A square with a plus sign, indicating addition or increase.",
            FAIcon::SquarePollHorizontal => {
                "A square with horizontal bars, representing a horizontal poll or chart."
            }
            FAIcon::SquarePollVertical => {
                "A square with vertical bars, representing a vertical poll or chart."
            }
            FAIcon::SquareRootVariable => {
                "A square with a variable inside a root symbol, representing mathematics."
            }
            FAIcon::SquareRss => "A square with RSS icon, representing news feed.",
            FAIcon::SquareShareNodes => {
                "A square with nodes connected by lines, indicating sharing or networking."
            }
            FAIcon::SquareSteam => "A square with the Steam logo, representing the gaming platform.",
            FAIcon::SquareUpRight => {
                "A square with a bold arrow pointing up and right, indicating expansion or exit."
            }
            FAIcon::SquareVirus => "A square with virus icons, representing illness or infection.",
            FAIcon::SquareXmark => "A square with an `X`, representing rejection or closure.",
            FAIcon::Squarespace => "The logo of Squarespace, a website building platform.",
            FAIcon::StackOverflow => "The logo of Stack Overflow, a Q&A platform for developers.",
            FAIcon::StaffSnake => "A staff with a snake, representing medical profession.",
            FAIcon::Stairs => "A staircase, representing steps or levels.",
            FAIcon::Stamp => "A stamp, representing approval or postage.",
            FAIcon::Stapler => "A stapler, representing office supplies.",
            FAIcon::Star => "A star, often used to represent favorites or ratings.",
            FAIcon::StarAndCrescent => "A star and crescent, representing Islam.",
            FAIcon::StarHalf => "A half-filled star, indicating partial rating.",
            FAIcon::StarHalfStroke => {
                "A half-filled star, indicating partial rating, with a stroke around it."
            }
            FAIcon::StarOfDavid => "A star of David, representing Judaism.",
            FAIcon::StarOfLife => {
                "A six-pointed star with a rod in the center, representing emergency medical services."
            }
            FAIcon::Steam => "The logo of Steam, representing the gaming platform.",
            FAIcon::SteamSymbol => {
                "The logo of Steam, representing the gaming platform, with white background."
            }
            FAIcon::SterlingSign => "The symbol for the British pound, indicating currency.",
            FAIcon::Stethoscope => "A stethoscope, representing medical examination or healthcare.",
            FAIcon::Stop => "A stop sign, indicating cessation or pause.",
            FAIcon::Stopwatch => "A simple stopwatch, representing timing.",
            FAIcon::Stopwatch20 => "A stopwatch showing 20 seconds, representing time measurement.",
            FAIcon::Store => "A storefront, representing retail or shops.",
            FAIcon::StoreSlash => "A store with a slash, indicating closed or no store.",
            FAIcon::StreetView => "A street view symbol, representing navigation or mapping.",
            FAIcon::Strikethrough => {
                "Text with a line through it, indicating deletion or correction."
            }
            FAIcon::Stripe => "The logo of Stripe, a payment processing platform.",
            FAIcon::StripeS => "The logo of Stripe, representing the payment processing platform.",
            FAIcon::Stroopwafel => "A stroopwafel, representing the Dutch treat.",
            FAIcon::Subscript => "A subscript 'A', indicating subscript text.",
            FAIcon::Suitcase => "A simple suitcase, representing travel or luggage.",
            FAIcon::SuitcaseMedical => "A medical suitcase, representing emergency medical kit.",
            FAIcon::SuitcaseRolling => "A suitcase with wheels, representing travel.",
            FAIcon::Sun => "A sun, representing daytime or brightness.",
            FAIcon::SunPlantWilt => "A sun with a wilted plant, indicating drought or plant stress.",
            FAIcon::Superscript => "A superscript 'A', indicating superscript text.",
            FAIcon::Swatchbook => "A swatchbook, representing color samples or design.",
            FAIcon::Synagogue => "A synagogue, representing a place of worship for Jews.",
            FAIcon::Syringe => "A syringe, representing medical injections.",
            FAIcon::T => "A capital letter 'T', representing the letter.",
            FAIcon::Table => "A simple table, indicating data or spreadsheet.",
            FAIcon::TableCells => "A table with cells, representing data organization.",
            FAIcon::TableCellsColumnLock => "A table with a locked column, indicating fixed data.",
            FAIcon::TableCellsLarge => "A table with large cells, representing data organization.",
            FAIcon::TableCellsRowLock => "A table with a locked row, indicating fixed data.",
            FAIcon::TableColumns => "A table with columns, representing data organization.",
            FAIcon::TableList => "A table with a list, representing data organization.",
            FAIcon::TableTennisPaddleBall => {
                "A table tennis paddle with a ball, representing the sport."
            }
            FAIcon::Tablet => "A tablet device, representing mobile computing.",
            FAIcon::TabletButton => "A tablet with a button, representing a touchscreen device.",
            FAIcon::TabletScreenButton => {
                "A tablet with a screen and button, representing a digital device."
            }
            FAIcon::Tablets => "Two pills, representing medication or tablets.",
            FAIcon::TachographDigital => "A digital tachograph, representing vehicle monitoring.",
            FAIcon::Tag => "A price tag, indicating labels or pricing.",
            FAIcon::Tags => "Multiple tags, representing labels or categories.",
            FAIcon::Tape => "A roll of tape, representing adhesive tape.",
            FAIcon::Tarp => "A simple tarp, representing a cover or protection.",
            FAIcon::TarpDroplet => "A tarp with a droplet, representing waterproof covering.",
            FAIcon::Taxi => "A taxi cab, representing transportation service.",
            FAIcon::Teeth => "A set of teeth, representing dental health.",
            FAIcon::TeethOpen => "An open mouth with teeth, representing dental health or smiling.",
            FAIcon::TemperatureArrowDown => {
                "A thermometer with a downward arrow, indicating falling temperature."
            }
            FAIcon::TemperatureArrowUp => {
                "A thermometer with an upward arrow, indicating rising temperature."
            }
            FAIcon::TemperatureEmpty => "A thermometer empty, representing no temperature reading.",
            FAIcon::TemperatureFull => "A thermometer full, representing very high temperature.",
            FAIcon::TemperatureHalf => "A thermometer half full, representing moderate temperature.",
            FAIcon::TemperatureHigh => {
                "A thermometer with high reading, indicating high temperature."
            }
            FAIcon::TemperatureLow => "A thermometer with low reading, indicating low temperature.",
            FAIcon::TemperatureQuarter => {
                "A thermometer one-quarter full, representing low temperature."
            }
            FAIcon::TemperatureThreeQuarters => {
                "A thermometer three-quarters full, representing high temperature."
            }
            FAIcon::TengeSign => "The symbol for the Kazakhstani tenge, indicating currency.",
            FAIcon::Tent => "A single tent, representing camping or temporary shelter.",
            FAIcon::TentArrowDownToLine => {
                "A tent with an arrow pointing down to a line, representing a campsite."
            }
            FAIcon::TentArrowLeftRight => {
                "A tent with arrows pointing left and right, indicating horizontal setup."
            }
            FAIcon::TentArrowTurnLeft => {
                "A tent with an arrow turning left, indicating directional setup."
            }
            FAIcon::TentArrowsDown => "A tent with arrows pointing down, indicating tent setup.",
            FAIcon::Tents => "Multiple tents, representing camping or temporary shelter.",
            FAIcon::Terminal => "A computer terminal, representing command line or coding.",
            FAIcon::TextHeight => "An icon indicating text height adjustment.",
            FAIcon::TextSlash => "Text with a slash, indicating no text.",
            FAIcon::TextWidth => "An icon indicating text width adjustment.",
            FAIcon::Thermometer => "A thermometer, representing temperature measurement.",
            FAIcon::ThumbsDown => "A thumbs-down gesture, indicating disapproval or dislike.",
            FAIcon::ThumbsUp => "A thumbs-up gesture, indicating approval or like.",
            FAIcon::Thumbtack => "A thumbtack, indicating pinned items or locations.",
            FAIcon::Ticket => "A ticket, representing admission or entry to an event.",
            FAIcon::TicketSimple => "A simple ticket, representing admission or entry.",
            FAIcon::Tiktok => "The logo of TikTok, a video-sharing social media platform.",
            FAIcon::Timeline => "A timeline, representing chronological events.",
            FAIcon::ToggleOff => "A switch in the off position, indicating deactivation.",
            FAIcon::ToggleOn => "A switch in the 'on' position, indicating activation.",
            FAIcon::Toilet => "A toilet, representing restrooms.",
            FAIcon::ToiletPaper => "A roll of toilet paper, indicating sanitation.",
            FAIcon::ToiletPaperSlash => {
                "A toilet paper roll with a slash, indicating no toilet paper."
            }
            FAIcon::ToiletPortable => "A portable toilet, representing outdoor facilities.",
            FAIcon::ToiletsPortable => {
                "Portable toilets, indicating temporary sanitation facilities."
            }
            FAIcon::Toolbox => "A toolbox, representing tools or repair.",
            FAIcon::Tooth => "A tooth, representing dentistry or oral health.",
            FAIcon::ToriiGate => "A torii gate, representing Japanese culture.",
            FAIcon::Tornado => "A tornado, representing severe weather.",
            FAIcon::TowerBroadcast => "A broadcast tower, representing media transmission.",
            FAIcon::TowerCell => "A cell tower, representing communication.",
            FAIcon::TowerObservation => "A tall observation tower, representing sightseeing.",
            FAIcon::Tractor => "A tractor, representing agriculture or farming.",
            FAIcon::Trademark => "A trademark symbol, representing brand or intellectual property.",
            FAIcon::TrafficLight => "A traffic light, representing road signals.",
            FAIcon::Trailer => "A trailer, representing cargo or transport.",
            FAIcon::Train => "A train, representing railway transport.",
            FAIcon::TrainSubway => "A subway train, representing underground transportation.",
            FAIcon::TrainTram => "A tram, representing public transportation.",
            FAIcon::Transgender => {
                "A combined male and female symbol, representing transgender identity."
            }
            FAIcon::Trash => "A trash can, representing deletion or garbage.",
            FAIcon::TrashCan => "A trash can, representing waste disposal.",
            FAIcon::Tree => "A tree, representing nature or the environment.",
            FAIcon::TreeCity => "A tree with a cityscape, representing urban nature or parks.",
            FAIcon::TriangleExclamation => {
                "A triangle with an exclamation mark, indicating warning or caution."
            }
            FAIcon::Trophy => "A trophy, representing achievement or awards.",
            FAIcon::Trowel => "A trowel, representing construction or gardening.",
            FAIcon::TrowelBricks => "A trowel with bricks, representing construction or masonry.",
            FAIcon::Truck => "An icon of a truck, indicating transportation or delivery.",
            FAIcon::TruckArrowRight => "A truck with an arrow pointing right, representing delivery.",
            FAIcon::TruckDroplet => "A truck with a droplet, representing liquid transport.",
            FAIcon::TruckFast => "A fast-moving truck, indicating quick delivery or shipment.",
            FAIcon::TruckField => "A truck in a field, indicating agricultural transport.",
            FAIcon::TruckFieldUn => {
                "A truck in a field, indicating agricultural transport, with UN letters on it."
            }
            FAIcon::TruckFront => "A front view of a truck, indicating transportation or delivery.",
            FAIcon::TruckMedical => "A medical truck, representing emergency medical transport.",
            FAIcon::TruckMonster => "A monster truck, indicating a large, powerful vehicle.",
            FAIcon::TruckMoving => "A moving truck, representing relocation or transport.",
            FAIcon::TruckPickup => "A pickup truck, representing a vehicle or transportation.",
            FAIcon::TruckPlane => "A truck with a plane, representing logistics.",
            FAIcon::TruckRampBox => "A truck with a ramp, indicating delivery or loading.",
            FAIcon::Tty => "An old-fashioned telephone with a keyboard, indicating teletype.",
            FAIcon::TurkishLiraSign => "The symbol for the Turkish lira, indicating currency.",
            FAIcon::TurnDown => "An arrow curving downwards, indicating turning down.",
            FAIcon::TurnUp => "An arrow curving upwards, indicating turning up.",
            FAIcon::Tv => "A television set, representing media or entertainment.",
            FAIcon::Twitch => "The logo of Twitch, a live streaming platform.",
            FAIcon::Twitter => "The logo of Twitter, a well-known social media platform.",
            FAIcon::U => "The letter \"U\", representing the alphabet.",
            FAIcon::Umbrella => "An umbrella, indicating protection from rain or sun.",
            FAIcon::UmbrellaBeach => "An umbrella on a beach, representing leisure or vacation.",
            FAIcon::Underline => "A line below text, indicating underline or emphasis.",
            FAIcon::UniversalAccess => "A circle with a person inside, representing accessibility.",
            FAIcon::Unlock => "An open padlock, indicating access or security.",
            FAIcon::UnlockKeyhole => "An open padlock with a keyhole, indicating access or security.",
            FAIcon::Unsplash => "The logo of Unsplash, representing the photo sharing platform.",
            FAIcon::UpDown => {
                "An arrow pointing up and another pointing down, indicating vertical movement."
            }
            FAIcon::UpDownLeftRight => {
                "Arrows pointing in all four directions, indicating omnidirectional movement."
            }
            FAIcon::UpLong => "A long arrow pointing up, indicating upward direction or increase.",
            FAIcon::UpRightAndDownLeftFromCenter => {
                "An arrow pointing up right and down left from center, indicating movement."
            }
            FAIcon::UpRightFromSquare => {
                "An arrow pointing up and right from a square, indicating expansion or exit."
            }
            FAIcon::Upload => "An arrow pointing upward from a box, indicating upload.",
            FAIcon::User => "An outline of a person, indicating a user or profile.",
            FAIcon::UserAstronaut => {
                "A user icon wearing an astronaut helmet, representing an astronaut or space exploration."
            }
            FAIcon::UserCheck => {
                "A user icon with a check mark, indicating user approval or verification."
            }
            FAIcon::UserClock => {
                "A user icon with a clock, indicating user schedule or time management."
            }
            FAIcon::UserDoctor => {
                "A user icon with a stethoscope, representing a doctor or healthcare professional."
            }
            FAIcon::UserGear => "A user icon with a gear, representing user settings or management.",
            FAIcon::UserGraduate => {
                "A user wearing a graduation cap, indicating education or graduation."
            }
            FAIcon::UserGroup => "Multiple user icons, indicating a group or community.",
            FAIcon::UserInjured => "A user with an injury, representing injury or accident.",
            FAIcon::UserLarge => "A large user icon, indicating a prominent user.",
            FAIcon::UserLargeSlash => {
                "A large user icon with a slash, representing user removal or restriction."
            }
            FAIcon::UserLock => "A user with a lock, representing account security.",
            FAIcon::UserMinus => "A user icon with a minus sign, indicating user removal.",
            FAIcon::UserNinja => {
                "A user icon with a ninja mask, indicating a stealthy or anonymous user."
            }
            FAIcon::UserNurse => "A user with a nurse hat, representing medical staff.",
            FAIcon::UserPen => "A user icon with a pen, representing user editing or writing.",
            FAIcon::UserPlus => "A user icon with a plus sign, indicating adding a user.",
            FAIcon::UserSecret => {
                "A user with a finger over their lips, indicating secrecy or confidentiality."
            }
            FAIcon::UserShield => "A user with a shield, representing user protection or security.",
            FAIcon::UserSlash => "A user icon with a slash, indicating a removed or blocked user.",
            FAIcon::UserTag => "A user with a tag, representing user identification.",
            FAIcon::UserTie => "A user icon with a tie, indicating a professional user.",
            FAIcon::UserXmark => "A user with a circled X, representing user removal.",
            FAIcon::Users => "Multiple user icons, representing a group or community.",
            FAIcon::UsersBetweenLines => {
                "Multiple user icons between lines, indicating collaboration or communication."
            }
            FAIcon::UsersGear => {
                "Multiple user icons with a gear, representing user settings or management."
            }
            FAIcon::UsersLine => "Multiple users in a line, representing a group or queue.",
            FAIcon::UsersRays => "Multiple users with rays, representing community or influence.",
            FAIcon::UsersRectangle => {
                "Multiple user icons inside a rectangle, representing a group or community."
            }
            FAIcon::UsersSlash => "Multiple user icons with a slash, indicating no users or blocked.",
            FAIcon::UsersViewfinder => {
                "A user icon inside a viewfinder, representing focus on users."
            }
            FAIcon::Utensils => "A fork and knife, representing dining or food.",
            FAIcon::V => "A capital letter 'V', representing the letter.",
            FAIcon::VanShuttle => "A shuttle van, representing transportation.",
            FAIcon::Vault => "A vault, representing security or storage.",
            FAIcon::VectorSquare => "A square with vector points, representing design or graphics.",
            FAIcon::Venus => "The symbol of Venus, representing the female gender.",
            FAIcon::VenusDouble => "Two Venus symbols, representing female gender or partnership.",
            FAIcon::VenusMars => "The symbols of Venus and Mars combined, representing gender.",
            FAIcon::Vest => "A vest, representing clothing.",
            FAIcon::VestPatches => "A vest with patches, representing protective gear.",
            FAIcon::Vial => "A vial, representing a small container for liquids.",
            FAIcon::VialCircleCheck => {
                "A vial with a circled check mark, representing approved substance."
            }
            FAIcon::VialVirus => "A vial with a virus, representing medical testing.",
            FAIcon::Vials => "Two laboratory vials, representing testing or experimentation.",
            FAIcon::Video => "A video camera, indicating video content or recording.",
            FAIcon::VideoSlash => {
                "A video symbol with a slash, indicating no video or disabled video."
            }
            FAIcon::Vihara => "A Buddhist temple, representing a place of worship.",
            FAIcon::Virus => "A virus, representing infection or disease.",
            FAIcon::VirusCovid => "A representation of the COVID-19 virus.",
            FAIcon::VirusCovidSlash => {
                "A virus symbol with a slash, representing COVID-19 eradication."
            }
            FAIcon::VirusSlash => "A virus with a slash, indicating antivirus or no virus.",
            FAIcon::Viruses => "Multiple virus icons, representing infections or disease.",
            FAIcon::Voicemail => "An icon of a cassette tape, representing voicemail messages.",
            FAIcon::Volcano => "A volcano, representing eruption or natural phenomenon.",
            FAIcon::Volleyball => "A volleyball, representing the sport.",
            FAIcon::VolumeHigh => "A speaker with high volume, representing loud audio.",
            FAIcon::VolumeLow => "A speaker with low volume, representing soft audio.",
            FAIcon::VolumeOff => "A speaker without sound waves, indicating no volume.",
            FAIcon::VolumeXmark => "A speaker with an `X`, indicating mute or no sound.",
            FAIcon::VrCardboard => "The logo of Google Cardboard, a VR platform.",
            FAIcon::W => {
                "A capital letter 'W', representing the letter or something starting with W."
            }
            FAIcon::WalkieTalkie => "A walkie-talkie, representing communication devices.",
            FAIcon::Wallet => "A wallet, representing money or finances.",
            FAIcon::WandMagic => "A magic wand, representing magical effects.",
            FAIcon::WandMagicSparkles => {
                "A magic wand with sparkles, indicating magical effects or settings."
            }
            FAIcon::WandSparkles => "A magic wand with sparkles, indicating magic or settings.",
            FAIcon::Warehouse => "A warehouse building, representing storage or logistics.",
            FAIcon::Water => "A water droplet, representing liquid or hydration.",
            FAIcon::WaterLadder => "A water ladder, representing swimming pools or rescue.",
            FAIcon::WaveSquare => "A square wave, representing a waveform.",
            FAIcon::WebAwesome => "The logo of Web Awesome, representing the web development tool.",
            FAIcon::WeightHanging => "A hanging weight, representing heavy lifting or measurement.",
            FAIcon::WeightScale => "A weight scale, representing measurement of weight.",
            FAIcon::WheatAwn => "A stalk of wheat, representing grain or agriculture.",
            FAIcon::WheatAwnCircleExclamation => {
                "A circle with wheat and an exclamation mark, representing gluten alert."
            }
            FAIcon::Wheelchair => "A simple wheelchair, representing accessibility.",
            FAIcon::WheelchairMove => "A wheelchair with motion lines, representing mobility.",
            FAIcon::WhiskeyGlass => "A whiskey glass, representing alcohol or beverages.",
            FAIcon::Wifi => "A signal icon, representing wireless internet connectivity.",
            FAIcon::Wind => "A wind symbol, representing breeze or weather.",
            FAIcon::WindowMaximize => "A window being maximized, representing expansion.",
            FAIcon::WindowMinimize => "A window being minimized, representing reduction.",
            FAIcon::WindowRestore => "A window being restored, representing reopening or resizing.",
            FAIcon::Windows => "The logo of Windows, a popular operating system by Microsoft.",
            FAIcon::WineBottle => "A wine bottle, representing alcohol or beverages.",
            FAIcon::WineGlass => "A wine glass, representing drinking or celebration.",
            FAIcon::WineGlassEmpty => "An empty wine glass, representing a drink.",
            FAIcon::WizardsOfTheCoast => {
                "The logo of Wizards of the Coast, representing the game company."
            }
            FAIcon::WonSign => "The symbol for the South Korean won, indicating currency.",
            FAIcon::Wordpress => "The logo of WordPress, a popular content management system.",
            FAIcon::Worm => "A worm, representing the animal or an insult.",
            FAIcon::Wrench => "A wrench, representing tools or repair.",
            FAIcon::X => "A capital letter `X`, representing the letter or close.",
            FAIcon::XRay => "An X-ray, representing medical imaging.",
            FAIcon::Xbox => "The logo of Xbox, representing the gaming console.",
            FAIcon::Xmark => "A simple `X` mark, indicating error or cancellation.",
            FAIcon::XmarksLines => "Lines forming an `X`, representing rejection or closure.",
            FAIcon::Y => "The letter `Y`, representing the alphabet.",
            FAIcon::YenSign => "The symbol for the Japanese yen, indicating currency.",
            FAIcon::YinYang => "A yin-yang symbol, representing balance or duality.",
            FAIcon::Youtube => "The logo of YouTube, a video-sharing platform.",
            FAIcon::Z => "A capital letter 'Z', representing the letter or sleep.",
        }
    }
}
