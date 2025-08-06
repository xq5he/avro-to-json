#!/usr/bin/env python3
"""
Script to create test Avro files for testing the avro-to-json converter.
Requires: pip install avro-python3
"""

import avro.schema
import avro.io
import avro.datafile
import io
import json
from datetime import datetime, date

def create_simple_user_avro():
    """Create a simple user record Avro file"""
    schema_json = {
        "type": "record",
        "name": "User",
        "fields": [
            {"name": "id", "type": "int"},
            {"name": "name", "type": "string"},
            {"name": "email", "type": ["null", "string"], "default": None},
            {"name": "age", "type": ["null", "int"], "default": None},
            {"name": "active", "type": "boolean", "default": True}
        ]
    }
    
    schema = avro.schema.parse(json.dumps(schema_json))
    
    # Create records
    records = [
        {"id": 1, "name": "Alice Johnson", "email": "alice@example.com", "age": 28, "active": True},
        {"id": 2, "name": "Bob Smith", "email": "bob@example.com", "age": 35, "active": False},
        {"id": 3, "name": "Charlie Brown", "email": None, "age": None, "active": True},
    ]
    
    # Write to file
    with open("test_users.avro", "wb") as f:
        writer = avro.io.DatumWriter(schema)
        file_writer = avro.datafile.DataFileWriter(f, writer, schema)
        for record in records:
            file_writer.append(record)
        file_writer.close()
    
    print("Created test_users.avro")

def create_complex_avro():
    """Create a more complex Avro file with arrays and nested records"""
    schema_json = {
        "type": "record",
        "name": "ComplexRecord",
        "fields": [
            {"name": "id", "type": "string"},
            {"name": "metadata", "type": {
                "type": "record",
                "name": "Metadata",
                "fields": [
                    {"name": "created", "type": "string"},
                    {"name": "version", "type": "int"}
                ]
            }},
            {"name": "tags", "type": {"type": "array", "items": "string"}},
            {"name": "properties", "type": {"type": "map", "values": "string"}},
            {"name": "score", "type": ["null", "double"], "default": None}
        ]
    }
    
    schema = avro.schema.parse(json.dumps(schema_json))
    
    records = [
        {
            "id": "rec-001",
            "metadata": {"created": "2024-01-15", "version": 1},
            "tags": ["important", "urgent"],
            "properties": {"category": "A", "priority": "high"},
            "score": 95.5
        },
        {
            "id": "rec-002", 
            "metadata": {"created": "2024-01-16", "version": 2},
            "tags": ["normal"],
            "properties": {"category": "B", "priority": "low", "notes": "test"},
            "score": None
        }
    ]
    
    with open("test_complex.avro", "wb") as f:
        writer = avro.io.DatumWriter(schema)
        file_writer = avro.datafile.DataFileWriter(f, writer, schema)
        for record in records:
            file_writer.append(record)
        file_writer.close()
    
    print("Created test_complex.avro")

def create_enum_union_avro():
    """Create Avro file with enums and unions"""
    schema_json = {
        "type": "record",
        "name": "Product",
        "fields": [
            {"name": "id", "type": "int"},
            {"name": "name", "type": "string"},
            {"name": "category", "type": {
                "type": "enum",
                "name": "Category",
                "symbols": ["ELECTRONICS", "BOOKS", "CLOTHING", "HOME"]
            }},
            {"name": "price", "type": ["null", "double"], "default": None},
            {"name": "description", "type": ["null", "string"], "default": None}
        ]
    }
    
    schema = avro.schema.parse(json.dumps(schema_json))
    
    records = [
        {
            "id": 1,
            "name": "Laptop",
            "category": "ELECTRONICS", 
            "price": 999.99,
            "description": "High-performance laptop"
        },
        {
            "id": 2,
            "name": "Python Book",
            "category": "BOOKS",
            "price": 29.99,
            "description": None
        },
        {
            "id": 3,
            "name": "T-Shirt",
            "category": "CLOTHING",
            "price": None,
            "description": "Cotton T-shirt"
        }
    ]
    
    with open("test_products.avro", "wb") as f:
        writer = avro.io.DatumWriter(schema)
        file_writer = avro.datafile.DataFileWriter(f, writer, schema)
        for record in records:
            file_writer.append(record)
        file_writer.close()
    
    print("Created test_products.avro")

if __name__ == "__main__":
    create_simple_user_avro()
    create_complex_avro()
    create_enum_union_avro()
    
    print("\nTest Avro files created successfully!")
    print("You can now test the converter with:")
    print("  cargo run -- -i test_users.avro --pretty")
    print("  cargo run -- -i test_complex.avro --array --pretty")
    print("  cargo run -- -i test_products.avro -o products.json")
