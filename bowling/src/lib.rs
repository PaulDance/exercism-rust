#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct BowlingGame {
    current_frame: u8,
    current_throw: u8,
    prev_throw: u8,
    last_throw: u8,
    striked_pins: u8,
    frames: Vec<BowlingFrame>,
    is_complete: bool,
}

#[derive(Debug, PartialEq)]
enum BowlingFrame {
    Open(u8, u8),
    Spare(u8, u8),
    Strike,
    Tenth(u8, u8, u8),
}

impl BowlingFrame {
    fn raw_score(&self) -> u8 {
        match self {
            Self::Open(o1, o2) => o1 + o2,
            Self::Spare(_, _) | Self::Strike => 10,
            Self::Tenth(t1, t2, t3) => t1 + t2 + t3,
        }
    }

    fn first_throw(&self) -> u8 {
        match *self {
            Self::Open(o1, _) => o1,
            Self::Spare(s1, _) => s1,
            Self::Strike => 10,
            Self::Tenth(t1, _, _) => t1,
        }
    }

    fn second_throw(&self) -> u8 {
        match *self {
            Self::Open(_, o2) => o2,
            Self::Spare(_, s2) => s2,
            Self::Strike => 0,
            Self::Tenth(_, t2, _) => t2,
        }
    }

    fn two_throws(&self) -> u8 {
        self.first_throw() + self.second_throw()
    }

    fn total_score(&self, frame2: Option<&Self>, frame3: Option<&Self>) -> u8 {
        self.raw_score()
            + match self {
                Self::Open(_, _) | Self::Tenth(_, _, _) => 0,
                Self::Spare(_, _) => match frame2 {
                    None => 0,
                    Some(f2) => f2.first_throw(),
                },
                Self::Strike => match frame2 {
                    None => 0,
                    Some(f2) => {
                        f2.two_throws()
                            + match f2 {
                                Self::Strike => match frame3 {
                                    None => 0,
                                    Some(f3) => f3.first_throw(),
                                },
                                _ => 0,
                            }
                    }
                },
            }
    }
}

impl BowlingGame {
    const MAX_PINS: u8 = 120;

    pub fn new() -> Self {
        Self {
            current_frame: 1,
            current_throw: 1,
            prev_throw: 0,
            last_throw: 0,
            striked_pins: 0,
            frames: Vec::new(),
            is_complete: false,
        }
    }

    pub fn roll(&mut self, pins: u8) -> Result<(), Error> {
        if self.is_complete {
            Err(Error::GameComplete)
        } else if pins > 10
            || self.striked_pins + pins > Self::MAX_PINS
            || self.current_frame != 10 && self.last_throw + pins > 10
        {
            Err(Error::NotEnoughPinsLeft)
        } else if self.current_throw == 1 {
            if pins == 10 && self.current_frame != 10 {
                self.frames.push(BowlingFrame::Strike);
                self.current_frame += 1;
            } else {
                self.prev_throw = pins;
                self.last_throw = pins;
                self.current_throw += 1;
            }

            self.striked_pins += pins;
            Ok(())
        } else if self.current_frame != 10 {
            if self.last_throw + pins == 10 {
                self.frames.push(BowlingFrame::Spare(self.last_throw, pins));
            } else {
                self.frames.push(BowlingFrame::Open(self.last_throw, pins));
            }

            self.striked_pins += pins;
            self.last_throw = 0;
            self.current_throw = 1;
            self.current_frame += 1;
            Ok(())
        } else if self.current_throw == 2 {
            if self.prev_throw + pins < 10 {
                self.frames
                    .push(BowlingFrame::Tenth(self.prev_throw, pins, 0));
                self.is_complete = true;
            } else {
                self.last_throw = pins;
                self.current_throw += 1;
            }

            self.striked_pins += pins;
            Ok(())
        } else if self.last_throw != 10
            && pins != 10
            && self.prev_throw + self.last_throw < 10
            && self.last_throw + pins > 10
            || self.prev_throw == 10
                && self.last_throw < 10
                && (pins == 10 || self.last_throw + pins > 10)
        {
            return Err(Error::NotEnoughPinsLeft);
        } else {
            self.frames
                .push(BowlingFrame::Tenth(self.prev_throw, self.last_throw, pins));
            self.striked_pins += pins;
            self.is_complete = true;
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete {
            None
        } else {
            Some(
                (0..self.frames.len())
                    .map(|i| {
                        self.frames
                            .get(i)
                            .unwrap()
                            .total_score(self.frames.get(i + 1), self.frames.get(i + 2))
                            as u16
                    })
                    .sum(),
            )
        }
    }
}
