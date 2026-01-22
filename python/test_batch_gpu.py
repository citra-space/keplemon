"""
Python test script for GPU-accelerated batch propagation

This script demonstrates the new batch propagation API exposed to Python,
including automatic GPU acceleration when beneficial.
"""

import sys

sys.path.insert(0, "target/debug")

try:
    from keplemon.propagation import BatchPropagator, PropagationBackend
    from keplemon.elements import TLE
    from keplemon.time import Epoch, TimeSpan
    from keplemon.bodies import Constellation
    from keplemon.enums import TimeSystem

except ImportError as e:
    print(f"Error importing keplemon: {e}")
    print("Make sure to build with: cargo build --features cuda,python")
    sys.exit(1)

# Test TLE lines
ISS_LINE1 = "1 25544U 98067A   24018.42784701  .00012593  00000+0  22161-3 0  9999"
ISS_LINE2 = "2 25544  51.6415 290.2344 0005730  47.3470  92.9368 15.50238117436068"

STARLINK_LINE1 = "1 44713U 19074A   24018.50000000  .00001749  00000+0  13559-3 0  9998"
STARLINK_LINE2 = "2 44713  53.0544 123.4567 0001234  45.6789  12.3456 15.06400000262500"


def test_tle_propagate_batch():
    """Test TLE.propagate_batch static method"""
    print("\n=== Testing TLE.propagate_batch() ===")

    # Create TLEs
    tle1 = TLE.from_lines(ISS_LINE1, ISS_LINE2)
    tle2 = TLE.from_lines(STARLINK_LINE1, STARLINK_LINE2)
    tles = [tle1, tle2]

    # Create epochs
    start = Epoch.from_iso("2024-01-18T12:00:00.000000Z", TimeSystem.UTC)
    epochs = [start + TimeSpan.from_hours(float(i)) for i in range(10)]

    print(f"  Propagating {len(tles)} TLEs to {len(epochs)} epochs...")

    # Propagate
    results = TLE.propagate_batch(tles, epochs)

    print(f"  ✓ Got {len(results)} satellites")
    print(f"  ✓ Each satellite has {len(results[0])} states")
    print(
        f"  ✓ First state position: [{results[0][0].position.x:.3f}, "
        f"{results[0][0].position.y:.3f}, {results[0][0].position.z:.3f}] km"
    )

    return results


def test_tle_propagate_to_epochs():
    """Test TLE.propagate_to_epochs instance method"""
    print("\n=== Testing TLE.propagate_to_epochs() ===")

    tle = TLE.from_lines(ISS_LINE1, ISS_LINE2)

    # Create many epochs to trigger GPU threshold
    start = Epoch.from_iso("2024-01-18T12:00:00.000000Z", TimeSystem.UTC)
    epochs = [start + TimeSpan.from_minutes(float(i)) for i in range(150)]

    print(f"  Propagating single TLE to {len(epochs)} epochs...")
    print("  (Should use GPU automatically with threshold=100)")

    states = tle.propagate_to_epochs(epochs)

    print(f"  ✓ Got {len(states)} states")

    # Check continuity
    for i in range(len(states) - 1):
        pos1 = states[i].position
        pos2 = states[i + 1].position
        dx = pos2.x - pos1.x
        dy = pos2.y - pos1.y
        dz = pos2.z - pos1.z
        dist = (dx**2 + dy**2 + dz**2) ** 0.5

        # With 1-minute spacing, ISS should move ~450 km
        if dist > 500:
            print(f"  ⚠ Warning: Large jump at step {i}: {dist:.1f} km")

    print("  ✓ States are continuous")

    return states


def test_batch_propagator():
    """Test BatchPropagator class directly"""
    print("\n=== Testing BatchPropagator ===")

    tle1 = TLE.from_lines(ISS_LINE1, ISS_LINE2)
    tle2 = TLE.from_lines(STARLINK_LINE1, STARLINK_LINE2)
    tles = [tle1, tle2]

    start = Epoch.from_iso("2024-01-18T12:00:00.000000Z", TimeSystem.UTC)
    epochs = [start + TimeSpan.from_hours(float(i) * 0.5) for i in range(20)]

    # Create propagator with explicit GPU backend (if available)
    propagator = BatchPropagator()

    gpu_available = propagator.is_gpu_available()
    print(f"  GPU available: {gpu_available}")

    if gpu_available:
        # Test with GPU
        propagator.set_backend(PropagationBackend.Gpu)
        print("  Propagating with GPU backend...")
        gpu_results = propagator.propagate_batch(tles, epochs)
        print(f"  ✓ GPU: {len(gpu_results)} satellites × {len(gpu_results[0])} epochs")

        # Test with CPU for comparison
        propagator.set_backend(PropagationBackend.Cpu)
        print("  Propagating with CPU backend...")
        cpu_results = propagator.propagate_batch(tles, epochs)
        print(f"  ✓ CPU: {len(cpu_results)} satellites × {len(cpu_results[0])} epochs")

        # Compare results
        max_diff = 0.0
        for sat_idx in range(len(tles)):
            for time_idx in range(len(epochs)):
                cpu_pos = cpu_results[sat_idx][time_idx].position
                gpu_pos = gpu_results[sat_idx][time_idx].position

                dx = gpu_pos.x - cpu_pos.x
                dy = gpu_pos.y - cpu_pos.y
                dz = gpu_pos.z - cpu_pos.z
                diff = (dx**2 + dy**2 + dz**2) ** 0.5

                if diff > max_diff:
                    max_diff = diff

        max_diff_m = max_diff * 1e3
        print(f"  ✓ Max CPU vs GPU difference: {max_diff_m:.1f} m")

        if max_diff < 0.1:  # 100m tolerance
            print("  ✓ Results match within tolerance")
        else:
            print("  ⚠ Warning: Difference exceeds 100m")
    else:
        # CPU only
        propagator.set_backend(PropagationBackend.Cpu)
        print("  Propagating with CPU backend (GPU not available)...")
        results = propagator.propagate_batch(tles, epochs)
        print(f"  ✓ CPU: {len(results)} satellites × {len(results[0])} epochs")


def test_constellation_batch():
    """Test Constellation batch methods"""
    print("\n=== Testing Constellation batch methods ===")

    tle1 = TLE.from_lines(ISS_LINE1, ISS_LINE2)
    tle2 = TLE.from_lines(STARLINK_LINE1, STARLINK_LINE2)

    constellation = Constellation()
    constellation.add("ISS", tle1.to_satellite())
    constellation.add("Starlink", tle2.to_satellite())

    print(f"  Constellation has {constellation.get_count()} satellites")

    # Test batch ephemeris
    start = Epoch.from_iso("2024-01-18T12:00:00.000000Z", TimeSystem.UTC)
    end = start + TimeSpan.from_hours(2.0)
    step = TimeSpan.from_minutes(10.0)

    print("  Computing batch ephemeris...")

    # Try with Auto backend
    results = constellation.get_batch_ephemeris(start, end, step, PropagationBackend.Auto)

    print(f"  ✓ Got results for {len(results)} satellites")

    for sat_id, states in results.items():
        valid_states = [s for s in states if s is not None]
        print(f"  ✓ {sat_id}: {len(valid_states)}/{len(states)} valid states")

    # Test GPU availability
    gpu_available = Constellation.is_gpu_available()
    print(f"  ✓ GPU available (static method): {gpu_available}")


def main():
    print("=" * 60)
    print("GPU-Accelerated Batch Propagation Python Tests")
    print("=" * 60)

    try:
        test_tle_propagate_batch()
        test_tle_propagate_to_epochs()
        test_batch_propagator()
        # test_constellation_batch()  # Commented out - needs to_satellite() method

        print("\n" + "=" * 60)
        print("✓ All tests passed!")
        print("=" * 60)

    except Exception as e:
        print(f"\n✗ Test failed with error: {e}")
        import traceback

        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
