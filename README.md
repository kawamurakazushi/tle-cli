# TLE CLI

A TLE (Two-line elements) to JSON Parser.

## Installation

### For Mac

```bash
brew tap kawamurakazushi/tap
brew install tle
```

## Usage

```bash
$ tle "ISS (ZARYA)
1 25544U 98067A   20045.18587073  .00000950  00000-0  25302-4 0  9990
2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.49165514212791"

{
  "argument_of_perigee": 264.606,
  "classification": "U",
  "drag_term": 0.000025302,
  "eccentricity": 0.0004885,
  "element_number": 999,
  "ephemeris_type": 0,
  "epoch": "20045.18587073",
  "first_derivative_mean_motion": 9.5e-6,
  "inclination": 51.6443,
  "international_designator": "98067A",
  "mean_anomaly": 207.3845,
  "mean_motion": 15.49165514,
  "name": "ISS (ZARYA)",
  "revolution_number": 21279,
  "right_ascension": 242.0161,
  "satellite_number": 25544,
  "second_derivative_mean_motion": 0.0
}
```
